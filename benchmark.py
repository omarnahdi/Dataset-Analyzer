import pandas as pd
import subprocess
import time
import shutil

file = "chicago crimes.csv"

def run_pandas(file):
    print("[ Pandas ]")
    start = time.perf_counter()
    df = pd.read_csv(file, low_memory=False)
    total_rows, total_cols = df.shape

    print(f"\n{'Column':<30} | {'Missing':>7} | {'Type':<11} | {'Min':>7} | {'Max':>7} | {'Mean':>7}")
    print("-" * 84)

    for col in df.columns:
        missing = df[col].isna().sum()
        if pd.api.types.is_numeric_dtype(df[col]):
            col_type = "numeric"
            col_min  = f"{df[col].min():>7.2f}"
            col_max  = f"{df[col].max():>7.2f}"
            col_mean = f"{df[col].mean():>7.2f}"
        else:
            col_type = "categorical"
            col_min = col_max = col_mean = "N/A"

        print(f"{col:<30} | {missing:>7} | {col_type:<11} | {col_min:>7} | {col_max:>7} | {col_mean:>7}")

    elapsed = time.perf_counter() - start
    print(f"\nTotal rows: {total_rows} | Columns: {total_cols}")
    return elapsed


def run_csvkit(file):
    print("\n[ csvkit — csvstat ]")
    if not shutil.which("csvstat"):
        print("  csvstat not found — install with: pip install csvkit")
        return None
    start = time.perf_counter()
    result = subprocess.run(
        ["csvstat", file],
        capture_output=True, text=True
    )
    elapsed = time.perf_counter() - start
    print(result.stdout)
    if result.returncode != 0:
        print(f"  Error: {result.stderr}")
        return None
    return elapsed


def run_duckdb(file):
    print("\n[ DuckDB CLI ]")
    if not shutil.which("duckdb"):
        print("  duckdb not found — download from https://duckdb.org/docs/installation")
        return None
    start = time.perf_counter()
    result = subprocess.run(
        ["duckdb", "-c", f"SELECT column_name, column_type, count, null_percentage, min, max, avg FROM (SUMMARIZE SELECT * FROM read_csv_auto('chicago crimes.csv'))"],
        capture_output=True, text=True,
        encoding="utf-8",      # ← fixes the cp1252 crash
        errors="replace"
    )
    elapsed = time.perf_counter() - start
    print(result.stdout)
    if result.returncode != 0:
        print(f"  Error: {result.stderr}")
        return None
    return elapsed


print(f"File: {file}\n")
print("=" * 84)

pandas_time  = run_pandas(file)
csvkit_time  = None
duckdb_time  = run_duckdb(file)

print("\n" + "=" * 84)
print("BENCHMARK SUMMARY")
print("=" * 84)

results = {
    "Pandas"   : pandas_time,
    "csvkit"   : csvkit_time,
    "DuckDB"   : duckdb_time,
    "RustSight": 5.57,
}

for name, t in results.items():
    if t is None:
        print(f"  {name:<12}: not installed")
    else:
        print(f"  {name:<12}: {t:.2f}s")

print()
available = {k: v for k, v in results.items() if v is not None}
if available:
    fastest = min(available, key=available.get)
    slowest = max(available, key=available.get)
    print(f"  Fastest : {fastest} ({available[fastest]:.2f}s)")
    print(f"  Slowest : {slowest} ({available[slowest]:.2f}s)")
    print()
    baseline = available.get("Pandas", next(iter(available.values())))
    for name, t in available.items():
        if name != "Pandas":
            ratio = baseline / t
            faster_or_slower = "faster" if ratio > 1 else "slower"
            print(f"  Pandas vs {name:<12}: {max(ratio, 1/ratio):.1f}x {faster_or_slower}")