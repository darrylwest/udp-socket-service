set nocompatible
set showmode
set mouse=a

set shiftwidth=4
set tabstop=4
set expandtab
set autoindent
set smartindent
set number
set ruler

map <Tab> <C-W>W
map <f5> :!make test<CR>
map <f6> :!git add . && git commit -v && git push
nnoremap tn :tabnext<CR>
nnoremap tp :tabprev<CR>

let @f = 'ggO//// @class CLASSNAME//// @author darryl.west <darryl.west@raincitysoftware.com>// @created :r !isodatekJo//?CLASSNAMEcw'
let @g = ':!go run %'
let @t = ':g/^    /s//\t/'

hi StatusLine ctermfg=15 guifg=#ffffff ctermbg=239 guibg=#4e004e cterm=bold gui=bold
hi StatusLineNC ctermfg=249 guifg=#424242 ctermbg=237 guibg=#3a3a3a cterm=none gui=none

" set the cursor underline on inactive window
augroup BgHightlight
  autocmd!
  autocmd WinEnter * set nocul
  autocmd WinLeave * set cul
augroup END
