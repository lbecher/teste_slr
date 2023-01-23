use std::fmt::Error;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Tokens {
    Mult,
    AbreP,
    FechaP,
    Id(String),
    Fim,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum NaoTerminais {
    SL,
    T,
    F,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Estados {
    I0,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Producoes {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Acoes {
    Empilha(Estados),
    Reduz(Producoes),
    Aceita,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum ElementosDaPilha {
    Tokens(Tokens),
    NaoTerminais(NaoTerminais),
    Estados(Estados),
}

#[derive(Debug, Clone)]
struct Sintatico {
    acoes: Vec<Acoes>,
    entrada: Vec<Tokens>,
    estado_atual: Estados,
    pilha: Vec<ElementosDaPilha>,
    posicao: usize,
}

impl Sintatico {
    fn inicializar(entrada: Vec<Tokens>) -> Self {
        let mut pilha: Vec<ElementosDaPilha> = Vec::new();
        pilha.push(ElementosDaPilha::Estados(Estados::I0));
        Sintatico {
            acoes: Vec::new(),
            entrada: entrada,
            estado_atual: Estados::I0,
            pilha: pilha,
            posicao: 0,
        }
    }

    fn analisar(&mut self) -> Result<Vec<ElementosDaPilha>, ()> {
        loop {
            println!("Testando {:?}", self.entrada[self.posicao]);
            if let Ok(acao) = self.obtem_acao() {
                self.aplica_acao(acao);
                println!("{:?}", self.pilha);
                if acao == Acoes::Aceita {
                    return Ok(self.pilha.to_vec());
                }
                self.posicao += 1;
            } else {
                println!("Token inesperado!");
                return Err(());
            }
        }
    }
    
    fn obtem_acao(&mut self) -> Result<Acoes, ()> {
        match self.estado_atual {
            Estados::I0 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Ok(Acoes::Empilha(Estados::I4)),
                    Tokens::FechaP => Err(()),
                    Tokens::Fim => Err(()),
                    Tokens::Id(_s) => Ok(Acoes::Empilha(Estados::I3)),
                    Tokens::Mult => Err(()),
                }
            },
            Estados::I1 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Err(()),
                    Tokens::FechaP => Err(()),
                    Tokens::Fim => Ok(Acoes::Aceita),
                    Tokens::Id(_s) => Err(()),
                    Tokens::Mult => Ok(Acoes::Empilha(Estados::I5)),
                }
            },
            Estados::I2 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Err(()),
                    Tokens::FechaP => Ok(Acoes::Reduz(Producoes::P1)),
                    Tokens::Fim => Ok(Acoes::Reduz(Producoes::P1)),
                    Tokens::Id(_s) => Err(()),
                    Tokens::Mult => Ok(Acoes::Reduz(Producoes::P1)),
                }
            },
            Estados::I3 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Err(()),
                    Tokens::FechaP => Ok(Acoes::Reduz(Producoes::P3)),
                    Tokens::Fim => Ok(Acoes::Reduz(Producoes::P3)),
                    Tokens::Id(_s) => Err(()),
                    Tokens::Mult => Ok(Acoes::Reduz(Producoes::P3)),
                }
            },
            Estados::I4 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Ok(Acoes::Empilha(Estados::I4)),
                    Tokens::FechaP => Err(()),
                    Tokens::Fim => Err(()),
                    Tokens::Id(_s) => Ok(Acoes::Empilha(Estados::I3)),
                    Tokens::Mult => Err(()),
                }
            },
            Estados::I5 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Ok(Acoes::Empilha(Estados::I4)),
                    Tokens::FechaP => Err(()),
                    Tokens::Fim => Err(()),
                    Tokens::Id(_s) => Ok(Acoes::Empilha(Estados::I3)),
                    Tokens::Mult => Err(()),
                }
            },
            Estados::I6 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Err(()),
                    Tokens::FechaP => Ok(Acoes::Reduz(Producoes::P2)),
                    Tokens::Fim => Ok(Acoes::Reduz(Producoes::P2)),
                    Tokens::Id(_s) => Err(()),
                    Tokens::Mult => Ok(Acoes::Reduz(Producoes::P2)),
                }
            },
            Estados::I7 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Err(()),
                    Tokens::FechaP => Ok(Acoes::Empilha(Estados::I8)),
                    Tokens::Fim => Err(()),
                    Tokens::Id(_s) => Err(()),
                    Tokens::Mult => Ok(Acoes::Empilha(Estados::I5)),
                }
            },
            Estados::I8 => {
                match self.entrada[self.posicao].clone() {
                    Tokens::AbreP => Err(()),
                    Tokens::FechaP => Ok(Acoes::Reduz(Producoes::P4)),
                    Tokens::Fim => Ok(Acoes::Reduz(Producoes::P4)),
                    Tokens::Id(_s) => Err(()),
                    Tokens::Mult => Ok(Acoes::Reduz(Producoes::P4)),
                }
            },
        }
    }

    fn aplica_acao(&mut self, acao: Acoes) {
        self.acoes.push(acao);
        match acao {
            Acoes::Aceita => {},
            Acoes::Empilha(estado) => {
                self.pilha.push(ElementosDaPilha::Estados(estado));
                self.estado_atual = estado;
            },
            Acoes::Reduz(_producao) => {
                println!("Não sei o que fazer!");
            },
        }
    }
}

fn main() {
    //   ((x + y) * z)
    let tokens = vec![
        Tokens::AbreP,
        Tokens::Id("x".to_string()),
        Tokens::FechaP,
        Tokens::Mult,
        Tokens::Id("y".to_string()),
        Tokens::Fim,
    ];

    let mut sintatico = Sintatico::inicializar(tokens);

    let saida = sintatico.analisar().unwrap();

    println!("{:?}", saida);
}
