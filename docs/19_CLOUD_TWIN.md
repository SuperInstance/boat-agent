# 19 — The Cloud Twin: Cloudflare Backup & Free Analysis

> **Target Audience:** Agents building sync, the Fleet role, and edge
> workers.
> **Purpose:** The cloud half of the data twin: durable backup, shore/phone
> access, and free-tier edge analysis. The boat is source of truth; the
> cloud is amplifier, never dependency, **never a down-link command path.**
> **Status:** Governing document · Research verified against Cloudflare
> docs 2026-07 (sources inline) · Date: 2026-07-19

---

## Verified quotas (Cloudflare official docs, fetched 2026-07)

| Resource | Free quota | Notes & source |
|----------|-----------|----------------|
| R2 storage | 10 GB-month total | [pricing](https://developers.cloudflare.com/r2/pricing/) — **egress genuinely $0**; overage $0.015/GB-mo billed standalone (no plan upgrade needed) |
| R2 ops | 1M Class A + 10M Class B /mo | multipart up to 10K parts/object; deletes + aborts free forever |
| D1 | 5M rows read + 100K rows written **/day**, 5 GB total | [pricing](https://developers.cloudflare.com/d1/platform/pricing/) — resets 00:00 UTC, hard errors at cap |
| Workers | 100K req/day, 10ms CPU | cron triggers included |
| Workers AI | **10,000 neurons/day, account-wide** | [pricing](https://developers.cloudflare.com/workers-ai/platform/pricing/) — hard stop at cap |
| Vectorize | 30M queried dims/mo + 5M stored dims | [pricing](https://developers.cloudflare.com/vectorize/platform/pricing/) — free tier committed ("always include prototyping") |
| KV | 100K reads, **1K writes/day** | use D1 instead for cursors |
| Queues | 10K ops/day (~3.3K msgs) | 24h retention on free |

**⚠ LLaVA is gone.** `@cf/llava-hf/llava-1.5-7b-hf` (the legacy docs'
vision model) no longer appears in the pricing table — deprecated. The
free-usable vision models now: **`@cf/meta/llama-3.2-11b-vision-instruct`**
(4,410 n/M input tokens) and **`@cf/moondream/moondream3.1-9B-A2B`**.
Embeddings: `@cf/baai/bge-base-en-v1.5` (768-dim), `@cf/baai/bge-m3`
(1,024-dim, cheapest/token). The legacy cloudflare-worker code in docs
00–03 must be updated on contact.

## The math (1 vessel vs 20)

Assumptions: ~3,000 frames/day @ ~500KB (1.5 GB/day), ~5K JSON
records/day, embeddings only for ~50 curated records/day/vessel, frames
bundled hourly into tar.zst for upload.

| Resource | 1 vessel | 20 vessels |
|----------|----------|------------|
| R2 storage 10 GB | ❌ gone in ~7 days | ❌ day 1 |
| R2 Class A | ✅ ~9K/mo | ✅ ~180K/mo |
| D1 writes 100K/day | ✅ ~10K/day | ❌ ~200K/day (2× over) |
| D1 storage 5 GB | ✅ ~0.7 GB/yr | ⚠️ over in ~4 months (needs retention) |
| Workers AI 10K n/day | ✅ ~700 sampled frames/day | ⚠️ ~35 keyframes/vessel/day (by design) |
| Vectorize stored 5M dims | ✅ ~4.4 years | ⚠️ ~month 7 |
| Workers 100K req/day | ✅ ~2K/day | ✅ ~30K/day |

**Where free breaks first, and the escape:**
1. **R2 storage, day ~7** — unavoidable if originals back up. Escape:
   pay R2-only overage, **$1.88/mo/vessel at 90-day cloud retention**
   (no plan upgrade). Free-only alternative: cloud keeps derived
   products only (~5% of volume) and originals stay boat+USB. Call:
   **pay the cents, keep everything else free.**
2. **Workers AI neurons, day 1** if naive. Escape: boat-side keyframe
   sampling (anomaly/catch/classifier-triggered frames only).
3. **D1 writes, ~10–15 vessels** — escape: Workers Paid **$5/mo**, the
   best-value upgrade (lifts KV, Vectorize, DO, requests too).
4. D1 storage, Vectorize dims — retention rollups; overage is sub-cent.

**Bottom line: 1 vessel ≈ $2/mo all-in; 20 vessels ≈ $45–50/mo.**

## Pipeline

```
BOAT (source of truth)                      CLOUDFLARE (amplifier)
VesselMemory (meta.db + blobs)
  sync agent:                                ① signed manifest POST → Worker /sync/begin
   - digest namespaces                       ② R2 multipart direct (S3 API), 8 MiB parts,
   - tar.zst hourly bundles                     per-vessel token, prefix-scoped, write-only
   - Ed25519-sign manifest                   ③ PUT manifest.json + .sig LAST (commit marker)
                    ▼
R2 `vessel-twin`
  vessels/<vid>/bundles/YYYY/MM/DD/<sha256>.tar.zst
  vessels/<vid>/frames/YYYY/MM/DD/HH/<frame_id>.png
  vessels/<vid>/records/YYYY/MM/DD/<batch_id>.jsonl.zst
  vessels/<vid>/manifests/<session_id>.json(+.sig)
  fleet/aggregates/<region_cell>/<month>.json   (vessels: read-only)
  Lifecycle: abort incomplete multipart after 7d (mandatory);
             expire bundles/ after `storage.cloud_keep_days` (default 90)
                    ▼
Cron Worker (15min): verify signature + sha256 → Queue
Queue consumer:      D1 rows → budget check (neuron ledger) →
                     bge embeds curated records → Vectorize upsert
                     llama-3.2-11b-vision on SAMPLED keyframes
API Worker:          /search → Vectorize → D1 hydrate → presigned R2 URLs
                     /vessel/<id>/status · /digest
```

**Upload protocol:** S3-compatible multipart direct-to-R2 with a
**per-vessel token scoped write-only to `vessels/<vid>/`** — no Worker in
the upload path (saves requests, free resumability). On reconnect:
`ListParts`, resume missing parts. Content-addressed keys make re-uploads
idempotent (`HeadObject` before `PUT`). **Manifest last = atomic commit;
a session without a manifest is invisible to the indexer.**

**D1 schema:**

```sql
vessels(vessel_id PK, pubkey TEXT, name, created_at);
sync_sessions(session_id PK, vessel_id, manifest_key, prev_manifest_hash,
              state, started_at, completed_at);        -- the resume cursor
objects(sha256 PK, vessel_id, r2_key, size, kind, session_id, verified, uploaded_at);
records(record_id PK, vessel_id, ts, type, lat, lon,
        summary, payload_json, frame_sha256 NULL, vec_id NULL, provenance);
neuron_ledger(day PK, vision_used INT, embed_used INT); -- free-tier budgeter
CREATE INDEX idx_records_vessel_ts ON records(vessel_id, ts);
CREATE INDEX idx_records_type ON records(type, ts);
```

**Vectorize:** one index `vessel-records` (768-dim cosine), **namespace
per vessel** (50K available). Embed only curated records — never raw
frames. Fleet index later for the anonymized-pattern track.

**Workers AI endpoints (free-tier-sized):**
- `POST /ai/frame` — llama-3.2-11b-vision on boat-pre-selected keyframes
  (~14 n/pass, ledger-gated; degrades to "caption skipped, still
  searchable by telemetry" at budget).
- `POST /ai/embed` — bge-base (~0.6 n/record).
- `GET /search` — embed query → Vectorize → D1 → presigned URLs.

Daily fleet budget: ~500 vision passes (7,000 n) + 1,000 embeds (600 n)
+ digest summaries (~100 n) ≈ 7,700/10,000. The neuron ledger gates
hard, because the free tier hard-fails at 10K.

## Sync protocol (docs/08-compatible)

- **Sync unit = the digest** (`VesselMemory.digest(range)`): manifest =
  `{vessel_id, session_id, prev_manifest_hash, namespace_digests, objects,
  cursor}`, Ed25519-signed. `prev_manifest_hash` chains sessions — the
  cloud copy is tamper-evident and gap-checkable, mirroring the black box.
- **Up-sync only for data.** Cloud is write-once, never authoritative:
  it may index, embed, caption — it never writes into a local namespace.
- **Down-sync = candidates only.** Fleet-namespace candidates (playbook
  candidates, vocab merges, aggregate patterns) ship down as signed
  bundles; local precedence applies (human > agent > fleet); candidates
  need local replay evidence to actuate (docs/07). Unchanged.
- **No cloud conflicts by construction.** Cloud stores versions keyed by
  content hash; the boat resolves. Last-writer-wins is forbidden.
- **Raw telemetry never syncs** — digests + curated windows only
  (docs/08 rule).

## Security model

- **Identity:** per-vessel Ed25519 keypair on the boat; pubkey in
  `vessels` table and `vessel.toml`. Bad signatures → quarantine prefix +
  escalation, never index.
- **Upload credentials:** prefix-scoped, write-only R2 tokens. A leaked
  token can spam its own prefix but cannot read others or delete.
  Revocation = delete token.
- **Read path:** Cloudflare Access (free ≤50 users) in front of the API
  Worker; frames via short-lived presigned URLs. No public buckets.
- **Fleet anonymization (later):** strip `vessel_id` → HMAC with fleet
  salt, bucket to 1°×1° cell + month, emit only patterns with **k ≥ 3
  vessels** per cell/month. Raw cross-vessel data is never exposed.
- **The cloud can never actuate.** No down-link command path exists. The
  envelope (docs/05) and the stage-gate (docs/07) are untouched by
  construction.

## Relationship to docs/18

The local twin is the source of truth and answers every query offline.
The cloud twin holds: originals (backup), the searchable index (shore
access), derived products (embeddings, captions), and fleet aggregates.
**Deletion rule:** local warm-tier blobs may be GC'd to cold only after
TWO verified copies exist (USB manifest-verified + R2 manifest-acked) —
enforced in GC code (docs/18, F3).

---

**Prev:** `docs/18_DATA_TWIN_STORAGE.md` · **Next:**
`docs/20_GUI_AND_PRESETS.md`
