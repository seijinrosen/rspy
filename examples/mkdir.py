from pathlib import Path

p = Path("mock_dir/mock_dir2")


p.mkdir(parents=False, exist_ok=False)
p.mkdir(parents=False, exist_ok=True)
p.mkdir(parents=True, exist_ok=False)
p.mkdir(parents=True, exist_ok=True)
