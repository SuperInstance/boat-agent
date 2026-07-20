# 18 — The Data Twin: Local Storage Architecture

> **Target Audience:** Agents building the memory layer, the cascade
> daemon, importers, and anyone touching persistent data.
> **Purpose:** The permanent home for the system's one irreplaceable
> asset: time-synced vessel data. Harnesses will change; this store must
> outlive them all.
> **Status:** Governing document — supersedes the `.openclaw/workspace`
> folder-as-database pattern · Research: REVIEW in docs/15 style, agent
> R&D 2026-07-19 (sources inline) · Date: 2026-07-19

---

## The premise

The `.openclaw/workspace` folder was a harness detail masquerading as a
data home: files appeared, agents read them, nobody owned integrity,
retention, or verification — and a cleanup tool nearly destroyed the
corpus (see docs/16 incident). The data IS the product: every frame is a
time-synced, GPS-anchored observation that becomes more valuable as it
accumulates (patterns, training sets, chart corrections, legal record).
Storage is therefore a **governed subsystem**, not a side effect.

Design constraints:

- **Boat realities:** power loss mid-write, vibrating USB ports,
  consumer SSDs, months offline.
- **Volume asymmetry:** ~1.4 GB/day of blobs vs ~1–2 MB/day of metadata
  (~0.5 GB/yr). The ratio (~1000:1) drives every decision below.
- **Query shapes:** time-range, spatial (lat/lon), depth-band,
  full-text over analysis, "give me the training set."
- **The GC contract** (docs/17): nothing deleted unread, nothing deleted
  unverifiable.
- **No magic numbers:** retention windows and cadences live in
  `vessel.toml` (AGENTS.md).

## The decision (research summary)

Evaluated: SQLite (WAL/FTS5/R\*Tree), DuckDB, Parquet+index, LanceDB,
TileDB, TimescaleDB. Sources: [SQLite WAL checksum behavior](https://avi.im/blag/2025/sqlite-wal-checksum/),
[DuckDB crash recovery](https://duckdb.org/docs/lts/guides/troubleshooting/crashes.html),
[DuckDB storage-compat notes](https://www.cnblogs.com/ytwang/p/18702350),
[DuckDB×Lance interop (May 2026)](https://duckdb.org/2026/05/21/test-driving-lance.html),
[Vector DB comparison 2026](https://letsdatascience.com/blog/vector-databases-compared-pinecone-qdrant-weaviate-milvus-and-more).

| Candidate | Verdict |
|-----------|---------|
| **SQLite (WAL, FTS5, R\*Tree)** | ✅ **System of record** — zero-dep, single-file, battle-tested on Windows, right query fit |
| **DuckDB** | ✅ **Read-only analytics sidecar** — scans SQLite + Parquet in one SQL surface; never the record (crash-tolerant, not crash-hardened; format churn) |
| **Parquet exports** | ✅ **Cold-tier + dataset format** — immutable, atomic-rename friendly, ML lingua franca |
| **LanceDB/Lance** | ⏳ Revisit in 12 months for the labeled-training-set layer (MVCC, multimodal, Rust-native, now DuckDB-interoperable) |
| **TileDB** | ❌ Overkill, refuted — we store frames+rows, not dense n-d arrays. No array workload exists here |
| **TimescaleDB** | ❌ Rejected by constraint — a Postgres server on a boat is an ops burden nothing justifies |

**One sentence of architecture:** frames are immutable content-addressed
files; everything *about* them lives forever in one SQLite database;
analytics happens in DuckDB against Parquet exports; nothing is deleted
without two-phase GC and two verified cold copies.

This extends `core/src/memory/mod.rs` (SQLite + CAS, trait-swappable)
rather than replacing it: the perception store is a namespace-aware
specialization of VesselMemory.

## Layout

```
memory/                           # vessel data_dir, from vessel.toml
├── meta.db                       # THE database. SQLite, WAL. Small forever.
├── blobs/
│   └── ab/cd/abcd1234….png       # content-addressed by SHA-256
├── exports/
│   └── 2026-07/frames.parquet    # monthly metadata export (no pixels)
├── manifests/
│   └── 2026-07-19.manifest.jsonl # per-day verification unit (USB-backup atom)
└── gc/
    └── pending.json              # two-phase GC staging
```

Naming: `meta.db` not `frames.db` — it holds records, notes, briefings,
labels, and provenance, not just frames. `manifests/` exist so a USB copy
is verifiable **without SQLite** (defense against the no-checksum
problem, below).

## Schema (meta.db)

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous  = FULL;      -- boat rule, not negotiable
PRAGMA busy_timeout = 5000;
PRAGMA foreign_keys = ON;

CREATE TABLE frames (            -- one row per captured screenshot
  frame_id     TEXT PRIMARY KEY, -- ULID: time-sortable, unique across vessels
  ts_utc       INTEGER NOT NULL, -- epoch ms (GPS time-synced)
  lat REAL, lon REAL,            -- nullable: GPS dropout is real at sea
  sog REAL, cog REAL,
  sha256       TEXT NOT NULL REFERENCES blobs(sha256),
  bytes        INTEGER NOT NULL,
  tier         TEXT NOT NULL DEFAULT 'hot',  -- hot|warm|cold|gone
  cadence      TEXT NOT NULL,                -- '30s'|'10min-canonical'
  novelty      REAL,                         -- M1 score, if computed
  keep_reason  TEXT,                         -- NULL=GC-eligible|'canonical'|'novel'|'labeled'|'human'
  display_geom TEXT                          -- JSON sidecar geometry
);
CREATE INDEX idx_frames_ts ON frames(ts_utc);
CREATE INDEX idx_frames_tier_ts ON frames(tier, ts_utc);
CREATE VIRTUAL TABLE frames_geo USING rtree(frame_id, min_lon, max_lon, min_lat, max_lat);

CREATE TABLE blobs (             -- CAS registry
  sha256 TEXT PRIMARY KEY, path TEXT NOT NULL, bytes INTEGER NOT NULL,
  tier TEXT NOT NULL DEFAULT 'hot', created INTEGER NOT NULL
);

CREATE TABLE echogram_records (  -- M10 canonical record
  frame_id TEXT PRIMARY KEY REFERENCES frames,
  ts_utc INTEGER NOT NULL,
  depth_top_m REAL, depth_bot_m REAL,        -- depth-band queries
  record_json TEXT NOT NULL,
  record_sha256 TEXT NOT NULL,               -- self-verifying on read
  vocab_terms TEXT, model TEXT, confidence REAL
);
CREATE INDEX idx_records_depth ON echogram_records(depth_top_m, depth_bot_m);

CREATE TABLE notes (             -- M1 notes; transient by default
  note_id TEXT PRIMARY KEY, ts_utc INTEGER NOT NULL,
  frame_id TEXT REFERENCES frames,
  body TEXT, novelty REAL, retained INTEGER DEFAULT 0
);
CREATE VIRTUAL TABLE notes_fts   USING fts5(body, content='notes', content_rowid='rowid');
CREATE VIRTUAL TABLE records_fts USING fts5(vocab_terms, record_json);

CREATE TABLE briefings (         -- H1 output; canonical, never GC'd
  briefing_id TEXT PRIMARY KEY, ts_utc INTEGER NOT NULL,
  period_start INTEGER, period_end INTEGER,
  body TEXT, body_sha256 TEXT, model TEXT
);

CREATE TABLE labels (            -- THE MONEY TABLE: future training sets
  frame_id TEXT REFERENCES frames, label TEXT, labeler TEXT,
  ts_utc INTEGER, provenance TEXT,           -- docs/08 rule 1
  PRIMARY KEY(frame_id, label, labeler)
);
```

Embeddings, when they arrive (docs/08 semantic query), go in the same
file via **sqlite-vec** — one backup story, one failure mode set. No
second database for vectors at <10M scale.

## Tiering & retention (encodes the docs/17 GC contract)

| Tier | Contents | Location | Lifetime |
|------|----------|----------|----------|
| **hot** | all frames + all metadata | laptop SSD | until evening final read |
| **warm** | canonical 10-min frames + novel/labeled frames + **all metadata, forever** | laptop SSD | `storage.warm_days` (default 90) from vessel.toml |
| **cold** | warm content past the window, **after two verified copies exist** (USB + Cloudflare twin) | external/archive | forever off-boat |
| **gone** | GC'd minute frames (post final read, unremarkable) | — | row kept as **tombstone** (`tier='gone'`, sha256 kept) so history stays explainable |

**Metadata never tiers.** At ~0.5 GB/yr it lives in `meta.db` for the
life of the vessel — every time/spatial/depth query is always answerable
locally, offline, even for frames whose pixels are on a shelf.

## Write path (the order is the durability)

1. Blob: temp file → fsync → **atomic rename** → fsync parent dir.
2. Row: inserted in ONE transaction, referencing the blob hash.
3. Canonical records carry `*_sha256` so corruption is *detectable on read*.
4. Startup reconciliation: orphan blobs (no row, >1h old) → delete; rows
   with missing files → `tier='gone'` + escalation event (degrade toward
   the human, prime directive 5).

## The three failure modes that actually bite

**F1. Power loss silently truncates WAL.** On a checksum mismatch,
SQLite recovery *silently discards the bad frame and every WAL frame
after it* — no error raised ([avi.im, 2025](https://avi.im/blag/2025/sqlite-wal-checksum/)).
Mitigations: `synchronous=FULL`; per-record `*_sha256`; nightly
`PRAGMA integrity_check` logged as a bus event; nightly `VACUUM INTO`
to a second volume (online, incremental-safe).

**F2. Blob/DB divergence.** Crash between blob write and row insert (or
mid-GC) → orphans or phantom rows. Mitigations: strict write order
above; reconciliation sweep; **two-phase GC** — staging in
`gc/pending.json` with a 24h grace period before unlink, so a mistaken
GC is reversible.

**F3. The cold copy silently rots.** exFAT USB + vibration = unfalsifiable
"backup." Mitigations: the day `manifest.jsonl` is the verification unit
(re-hash every file after copy; manifests hash-chained via
`prev_manifest_sha256`); archive drives formatted **NTFS**, not exFAT;
**two verified cold copies before any local deletion — enforced in GC
code, not in docs.**

## Honest note on CAS

Content addressing buys dedup we will never use — no two echogram frames
hash alike. Keep it for **immutability, self-verification, and atomic
rename semantics**. Skip refcount machinery beyond the reconciliation
sweep; refcounts for a write-once asset class are ceremony.

## Query interface

- Agents: `boatctl memory query` compiles structured filters to this
  schema (docs/08 interface, unchanged).
- Analytics: DuckDB read-only, `sqlite_scanner` on `meta.db` +
  `read_parquet('exports/**/*.parquet')` — one SQL surface over hot and
  cold, zero extra infrastructure, no writes through DuckDB.

## 12-month revisit list

1. **Lance** for the labeled-training-set export (once `labels` has
   volume). It's a dataset-format decision, and the `exports/` boundary
   makes it a non-migration.
2. **sqlite-vec embeddings** when semantic query ships.
3. Whether monthly Parquet becomes canonical cold metadata and SQLite
   slims to hot-only. Don't preemptively optimize.

## Migration from the workspace folder

1. `B-INT-1` importer walks `captures/v3/**`: hash each PNG → `blobs/`,
   row per frame (sidecar → columns), record per existing analysis.
2. Cascade daemon switches its reads to `meta.db` + `blobs/` (its
   outputs were already records/notes/briefings-shaped).
3. The old folder becomes a read-only import source, then cold-tier
   material itself. The `.openclaw/workspace` path never appears in a
   config again.

---

**Next:** `docs/19_CLOUD_TWIN.md` — the other half of the twin.
