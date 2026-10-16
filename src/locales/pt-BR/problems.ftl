## Problemas de pacote (especificação de pacotes). Todos têm $file e $path:
## o arquivo dentro do pacote e o lugar nele.

problem-unreadable = Não foi possível ler o pacote: { $reason }
problem-missing-file = Falta { $file }.
problem-unknown-folder = { $file } não é uma pasta de conteúdo.
problem-unexpected-file = { $file } não deveria estar ali.
problem-syntax = { $file } não é um JSON válido (linha { $line }, coluna { $column }).
problem-not-an-entity = { $file } precisa conter um objeto ou uma lista de objetos.
problem-unknown-field = { $file }, { $path }: campo desconhecido "{ $field }".
problem-missing-field = { $file }, { $path }: falta o campo "{ $field }".
# $detail é a descrição do próprio leitor de JSON, em inglês.
problem-invalid-value = { $file }, { $path }: valor inválido ({ $detail }).
problem-invalid-format = { $file }, { $path }: o valor está no formato errado.
problem-out-of-range = { $file }, { $path }: o valor está fora do intervalo.
problem-invalid-id = { $file }, { $path }: "{ $id }" não é um id válido.
problem-wrong-kind = { $file }, { $path }: "{ $id }" pertence a uma pasta de { $expected }.
problem-duplicate-id = { $file }, { $path }: "{ $id }" também é definido em { $other }.
problem-unresolved-reference = { $file }, { $path }: nada define "{ $id }".
problem-wrong-count = O pacote precisa definir exatamente um { $kind }, não { $count }.
problem-unsupported-genre = { $file }: o gênero "{ $genre }" não é suportado.
problem-unknown-locale = { $file }: o idioma "{ $locale }" não está declarado no manifesto.
problem-text-syntax = { $file }, linha { $line }: o texto não é Fluent válido.
problem-missing-text = { $file }, { $path }: falta a mensagem "{ $message }" em { $locale }.
problem-invalid-mix = { $file }: a região { $region } não tem nacionalidade para sortear.
problem-empty-given-names = { $file }: { $nationality } não tem prenomes.
problem-empty-family-names = { $file }: { $nationality } não tem sobrenomes.
problem-no-roles = { $file }: o conjunto de funções não tem funções.
problem-duplicate-role = { $file }: a função { $role } aparece duas vezes.
problem-missing-weight = { $file }: a função { $role } não tem peso para { $attribute }.
problem-zero-weights = { $file }: os pesos de { $role } somam zero.
problem-too-many-subs = { $file }: { $subs } reservas, mas só { $roles } funções.
problem-duplicate-nickname = { $file }, { $path }: o apelido "{ $nickname }" também está fixado em { $other }.
problem-nickname-pool = { $file }: { $distinct } apelidos distintos, mas são necessários { $needed }.
problem-incompletable-roster = { $file }: os jogadores fixados de { $team } não deixam espaço para { $roles }.
