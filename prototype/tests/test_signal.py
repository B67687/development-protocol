# Stencil - G3 "chain seeable -> walk in <2 min" via trace
# Runnable: ./build_shim.sh  or: uv run pytest


def test_p2b_whctx_trace_exists():
    text = open('docs/traces/colour-blind-85-100.md').read()
    assert 'WHICH-X' in text and 'more-than-X' in text


def test_p2b_mapping_appendix_exists():
    text = open('docs/appendix/p2b-mapping-appendix.md').read()
    assert 'P2b Mapping' in text


def test_p4_late_appendix_exists():
    text = open('docs/appendix/p4-late-appendix.md').read()
    assert 'P4 Late' in text
