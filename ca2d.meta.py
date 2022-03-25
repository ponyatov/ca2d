from metaL import *
p = Project()
p.TITLE = '2D Web CAD engine'
p | metaL() | Rust() | Web()
p.sync()
