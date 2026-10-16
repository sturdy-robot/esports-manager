## Erros do Novo Jogo, dos jogos salvos e das consultas.

error-manager-name-empty = Digite o nome do treinador.
error-manager-name-long = O nome do treinador pode ter no máximo { $max } caracteres.
error-save-name-empty = Digite o nome do jogo salvo.
error-save-name-long = O nome do jogo salvo pode ter no máximo { $max } caracteres.
# $field é "manager" ou "save".
error-control-characters = { $field ->
    [manager] O nome do treinador
   *[save] O nome do jogo salvo
} não pode ter caracteres de controle.
error-unknown-team = Não existe o time "{ $team }".
error-unknown-player = Não existe o jogador "{ $player }".
error-generation = Não foi possível gerar o mundo a partir deste pacote.
error-save-name-taken = Já existe um jogo salvo chamado "{ $name }".
error-save-not-found = Não existe o jogo salvo "{ $file }".
error-save-not-a-save = Este arquivo não é um jogo salvo.
error-save-newer = Este jogo foi salvo por uma versão mais nova do jogo.
error-save-in-use = Este jogo salvo está aberto em outro lugar.
error-storage = Não foi possível ler ou gravar o jogo salvo: { $detail }
