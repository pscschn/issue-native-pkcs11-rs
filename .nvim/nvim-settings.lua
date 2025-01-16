local dap = require 'dap.rust'
dap.config()
dap.install()
local lsp = require 'lsp.rust'
lsp.config()
lsp.install()
