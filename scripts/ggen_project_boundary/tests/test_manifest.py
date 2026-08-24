import tempfile
import unittest
from pathlib import Path
from scripts.ggen_project_boundary.manifest import inspect_manifest
from scripts.ggen_project_boundary.types import Subject

class T(unittest.TestCase):
    def test_manifest(self):
        with tempfile.TemporaryDirectory() as td:
            root=Path(td); (root/"ggen").mkdir(); (root/"q.rq").write_text("SELECT * WHERE {}") ; (root/"t.tera").write_text("x")
            (root/"ggen/ggen.toml").write_text('[generation]\noutput_dir="."\n[[generation.rules]]\nname="x"\nquery={file="q.rq"}\ntemplate={file="t.tera"}\noutput_file="src/x.rs"\n')
            obs=inspect_manifest(root,Subject("seanchatmangpt/wasm4pm-compat","a"*40,"b"*40))
            self.assertEqual(obs["rules"][0]["target"]["contained"],True)
            self.assertNotEqual(obs["rules"][0]["query_digest"],"MISSING")
