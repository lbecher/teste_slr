use debug_print::debug_println;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Tokens {
    Mult,
    AbreP,
    FechaP,
    Id,
    Fim,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum NaoTerminais {
    SL,
    T,
    F,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Acoes {
    Empilha(usize),
    Reduz(usize),
    VaiPara(usize),
    Aceita,
    Erro,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum ElementosDaPilha {
    Tokens(Tokens),
    NaoTerminais(NaoTerminais),
    Estados(usize),
}

fn main() {
    //   (id)*id
    let tokens = vec![
        Tokens::AbreP,
        Tokens::Id,
        Tokens::FechaP,
        Tokens::Mult,
        Tokens::Id,
        Tokens::Fim,
    ];

    let mut sintatico = Sintatico::inicializar(&tokens);

    sintatico.analisar();
}


#[derive(Debug, Clone)]
struct Sintatico {
    entrada: Vec<Tokens>,
    pilha: Vec<ElementosDaPilha>,
    simbolo_anterior: ElementosDaPilha,
    simbolo_atual: ElementosDaPilha,
    vai_para: bool,
    modo_panico: bool,
}

impl Sintatico {
    fn inicializar(entrada: &Vec<Tokens>) -> Self {
        let mut pilha: Vec<ElementosDaPilha> = Vec::new();

        // adiciona estado 0 à pilha
        pilha.push(ElementosDaPilha::Estados(0));

        Sintatico {
            entrada: entrada.to_vec(),
            pilha: pilha,
            simbolo_anterior: ElementosDaPilha::Estados(0),
            simbolo_atual: ElementosDaPilha::Tokens(entrada[0].clone()),
            vai_para: false,
            modo_panico: false,
        }
    }

    fn analisar(&mut self) {
        // vetor de tuplas que possui o não terminal e a quantidade de itens de cada produção da gramáticas
        let producoes = vec![
            (NaoTerminais::SL, 1 as usize),
            (NaoTerminais::T, 1 as usize),
            (NaoTerminais::T, 3 as usize),
            (NaoTerminais::F, 1 as usize),
            (NaoTerminais::F, 3 as usize),
        ];

        println!("---------------------------------------\nIniciando análise sintática...\n-------------\n");

        // coloca primeiro token no símbolo atual
        self.simbolo_atual = ElementosDaPilha::Tokens(self.entrada[0].clone());

        loop {
            debug_println!("Pilha: {:?}\nEntrada: {:?}", self.pilha, self.entrada);

            // obtem ação com base na tabela SLR
            if let Ok(acao) = self.obtem_acao()
            {
                debug_println!("Ação: {:?}\n", acao);

                if acao == Acoes::Aceita
                {
                    if self.modo_panico == true {
                        println!("-------------\nERRO SINTÁTICO: Token(s) inesperado(s) encontrado(s)!\n---------------------------------------\n");
                    } else {
                        println!("-------------\nAnálise sintática terminou sem erros.\n---------------------------------------\n");
                    }
                    break;
                }
                else if let Acoes::Empilha(estado) = acao 
                {
                    // empilha símbolo atual na pilha
                    self.pilha.push(self.simbolo_atual.clone());

                    // empilha estado
                    self.pilha.push(ElementosDaPilha::Estados(estado));
                }
                else if let Acoes::Reduz(producao) = acao
                {
                    // preserva não terminal
                    self.simbolo_anterior = self.simbolo_atual.clone();

                    // elimina elementos da pilha de acordo com o número de elementos da produção * 2
                    for _i in 0..(producoes[producao].1 + producoes[producao].1) {
                        self.pilha.pop();
                    }

                    // coloca o não terminal obtido da produção no símbolo atual
                    self.simbolo_atual = ElementosDaPilha::NaoTerminais(producoes[producao].0);

                    // empilha o não terminal
                    self.pilha.push(ElementosDaPilha::NaoTerminais(producoes[producao].0));
                    
                    // ativa o modo vai para
                    self.vai_para = true;
                }
                else if let Acoes::VaiPara(estado) = acao
                {
                    // empilha novo estado
                    self.pilha.push(ElementosDaPilha::Estados(estado));

                    // restaura não terminal
                    self.simbolo_atual = self.simbolo_anterior.clone();

                    // desativa modo vai para
                    self.vai_para = false;
                }
                else // Acoes::Erro
                {
                    println!("-------------\nERRO: Token '{:?}' inesperado!\n-------------\n", self.simbolo_atual);

                    // ativa modo pânico
                    self.modo_panico = true;
                    
                    // remove token da entrada
                    self.entrada.remove(0);

                    // adiciona próximo token ao símbolo atual
                    self.simbolo_atual = ElementosDaPilha::Tokens(self.entrada[0].clone());
                }

                // verifica se token já foi adicionado à pilha
                let index = self.pilha.len() - 2;
                if let ElementosDaPilha::Tokens(token) = self.pilha[index]  {
                    if ElementosDaPilha::Tokens(token) == self.simbolo_atual {
                        // remove token da entrada
                        self.entrada.remove(0);

                        // adiciona próximo token ao símbolo atual
                        self.simbolo_atual = ElementosDaPilha::Tokens(self.entrada[0].clone());
                    }
                }
            }
            else
            {
                println!("-------------\nERRO INTERNO NO ANALISADOR SINTÁTICO!!!\n---------------------------------------\n");
                break;
            }
        }
    }

    fn obtem_acao(&mut self) -> Result<Acoes, ()> {
        // estado
        let estado: usize;
        let index_estado: usize;

        if self.vai_para == false {
            index_estado = self.pilha.len() - 1;
        } else {
            index_estado = self.pilha.len() - 2;
        }

        if let ElementosDaPilha::Estados(e) = self.pilha[index_estado] {
            estado = e;
        } else {
            println!("ERRO: O elemento na pilha não é um estado!");
            return Err(());
        }

        // símbolo
        let simbolo: ElementosDaPilha;

        if let ElementosDaPilha::Tokens(s) = self.simbolo_atual {
            simbolo = ElementosDaPilha::Tokens(s);
        } else if let ElementosDaPilha::NaoTerminais(s) = self.simbolo_atual {
            simbolo = ElementosDaPilha::NaoTerminais(s);
        } else {
            println!("ERRO: O elemento na pilha não é um símbolo de produção!");
            return Err(());
        }

        // tabela SRL
        match estado {
            0 => {
                if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Empilha(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Empilha(6));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::T) = simbolo {
                    return Ok(Acoes::VaiPara(1));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::F) = simbolo {
                    return Ok(Acoes::VaiPara(4));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            1 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Empilha(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Aceita);
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            2 => {
                if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Empilha(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Empilha(6));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::F) = simbolo {
                    return Ok(Acoes::VaiPara(3));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            3 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(2));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            4 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(1));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            5 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(3));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            6 => {
                if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Empilha(5));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Empilha(6));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::T) = simbolo {
                    return Ok(Acoes::VaiPara(7));
                } else if let ElementosDaPilha::NaoTerminais(NaoTerminais::F) = simbolo {
                    return Ok(Acoes::VaiPara(4));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            7 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Empilha(2));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Empilha(8));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            8 => {
                if let ElementosDaPilha::Tokens(Tokens::Mult) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Id) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::AbreP) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::FechaP) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else if let ElementosDaPilha::Tokens(Tokens::Fim) = simbolo {
                    return Ok(Acoes::Reduz(4));
                } else {
                    return Ok(Acoes::Erro);
                }
            },
            _ => {
                return Err(());
            },
        }
    }
}