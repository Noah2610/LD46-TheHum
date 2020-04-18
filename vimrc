nmap <leader>r :silent !RUN_TERMINAL=1 bin/run<CR>

autocmd BufNewFile,BufRead *.tsx,*.tmx setlocal syntax=xml filetype=xml
autocmd BufNewFile,BufRead *.yml       setlocal shiftwidth=2 softtabstop=2 tabstop=2
