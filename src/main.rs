use std::slice::EscapeAscii;

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
    acoes: Vec<Acoes>,
    entrada: Vec<Tokens>,
    pilha: Vec<ElementosDaPilha>,
    simbolo_anterior: ElementosDaPilha,
}

impl Sintatico {
    fn inicializar(entrada: &Vec<Tokens>) -> Self {
        let mut pilha: Vec<ElementosDaPilha> = Vec::new();
        pilha.push(ElementosDaPilha::Estados(0));
        Sintatico {
            acoes: Vec::new(),
            entrada: entrada.to_vec(),
            pilha: pilha,
            simbolo_anterior: ElementosDaPilha::Estados(0),
        }
    }

    fn analisar(&mut self) {
        // possui o não terminal e a quantidade de itens de cada produção da gramaticas
        let producoes = vec![
            (NaoTerminais::SL, 1 as usize),
            (NaoTerminais::T, 1 as usize),
            (NaoTerminais::T, 3 as usize),
            (NaoTerminais::F, 1 as usize),
            (NaoTerminais::F, 3 as usize),
        ];

        self.pilha.push(ElementosDaPilha::Tokens(self.entrada.remove(0).clone()));

        loop {
            println!("Pilha: {:?}\nEntrada: {:?}", self.pilha, self.entrada);

            if let Ok(acao) = self.obtem_acao() {
                println!("Ação: {:?}\n", acao);

                if acao == Acoes::Aceita {
                    println!("Deu tudo certo!");
                    break;
                }
                if acao == Acoes::Erro {
                    println!("Vish!");
                    break;
                }

                if let Acoes::VaiPara(estado) = acao {
                    let i = self.pilha.len() - 2;
                    let j = self.pilha.len() - 1;
                    self.pilha[i] = ElementosDaPilha::Estados(estado);
                    self.pilha[j] = self.simbolo_anterior.clone();
                } else if let Acoes::Empilha(estado) = acao {
                    self.pilha.push(ElementosDaPilha::Estados(estado));
                    self.pilha.push(ElementosDaPilha::Tokens(self.entrada.remove(0).clone()));
                } else if let Acoes::Reduz(producao) = acao {
                    // se prepara para a ação vai para
                    self.simbolo_anterior = self.pilha[self.pilha.len() - 1].clone();
                    // elimina elementos da pilha
                    for _i in 0..(producoes[producao].1 + producoes[producao].1) {
                        self.pilha.pop();
                    }
                    // empilha estado e o não terminal
                    if let ElementosDaPilha::Estados(estado) = self.pilha[self.pilha.len() - 2] {
                        self.pilha.push(ElementosDaPilha::Estados(estado));
                    }
                    self.pilha.push(ElementosDaPilha::NaoTerminais(producoes[producao].0));
                }
            } else {
                println!("Deu ruim no negócio tudo!");
                break;
            }
        }
    }

    fn obtem_acao(&mut self) -> Result<Acoes, ()> {
        let mut estado: usize;
        let mut simbolo: ElementosDaPilha;

        if let ElementosDaPilha::Estados(e) = self.pilha[self.pilha.len() - 2] {
            estado = e;
        } else {
            return Err(());
        }

        if let ElementosDaPilha::Tokens(s) = self.pilha[self.pilha.len() - 1] {
            simbolo = ElementosDaPilha::Tokens(s);
        } else if let ElementosDaPilha::NaoTerminais(s) = self.pilha[self.pilha.len() - 1] {
            simbolo = ElementosDaPilha::NaoTerminais(s);
        } else {
            return Err(());
        }

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