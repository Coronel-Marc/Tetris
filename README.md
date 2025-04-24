# ijatris

Ijatetris é um acronimo (Se é que posso chamar assim) para It's Just Another Tetris Game. Meu objetivo é desenvolver um jogo do tetris utilizando a linguagem Rust e o Bevy engine para praticar o que estarei aprendendo com leitura do O livro, tanto a versão traduzida quanto a original.

A medida que o projeto for crescendo irei atualizar este README, tornando-o uma espécie de historico resumido.

## Objetivo

 - Desenvolver um game 2D similar ao tetris utilizando a linguagem Rust.
 - Subir o maximo do projeto utilizando o Git via bash (sem GUI).

## Lista de tarefas

- [ X ] Janela para visualização
- [ X ] Gridline
- [ X ] Unidade do bloco
- [ ] Bloco Tetromino ( Formado por pelo menos 4 unidades)
- [ ] ...

## Resumo até agora

Tive muita dificuldade pra entender a lógica por trás de um jogo que eu considerava simples (rapaz...). 
Meu primeiro perrengue (dificuldade, dialeto local) foi sobre entender como fazer um tetromino. Mas antes disso eu precisava de uma estrutura que servisse de guia, então com a ajuda de um GPT da vida fui seguindo suas dicas. O objetivo nesse momento não era fazer uma Ctrl-C e V do que o chat dizia, mas sim seguir sua logica de desenvolvimento. 
Então decidi fazer a tal gridline para servir de mapeamento da tela, mais ela servirá também para identificar colisões, etc.

Atualmente (23/04/25) enfrento a dificuldade de renderizar um tetromino. Ja tenho a gridline e um unidade do bloco (um mino), desenvolvi a lógica de como montar um tetromino utilizando o mino mas não consigo fazer aparecer na tela o bendito tetromino...