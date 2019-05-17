#![feature(proc_macro_hygiene, decl_macro)]
mod cidade;
mod previsao;

use cidade::Cidade;
use previsao::Previsao;

use xml::reader::{EventReader, XmlEvent};

#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket::response::{content, Redirect};
use rocket::request::*;

#[derive(Debug, Clone, FromForm)]
struct Search {
    q: String,
}

fn main() {
    rocket::ignite().mount("/", routes![index, pesquisa, previsao, busca]).launch();
}

#[post("/", data = "<q>")]
fn busca(q: Form<Search>) -> Redirect {
    Redirect::to(uri!(pesquisa: q.clone().q))
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(format!(r#"<html>
                <head>
                    <title>Rocket Tempo</title>
                </head>
                <body>
                    <h1>Rocket Tempo</h1>
                    <form action="/" method="post" accept-charset="utf-8">
                        <input type="text" name="q"></input>
                        <input type="submit" value="Pesquisar">
                    </form>
                </body>
            </html>"#)
    )
 
}

#[get("/previsao?<id>")]
fn previsao(id: &RawStr) -> content::Html<String> {
    let s = format!{"http://servicos.cptec.inpe.br/XML/cidade/7dias/{}/previsao.xml", id.as_str()};
    let procura = reqwest::get(&s)
        .unwrap()
        .text()
        .unwrap();

    let parser = EventReader::from_str(&procura);
    let mut ele = String::new();

    let mut nome = String::new();
    let mut uf = String::new();

    let mut dia = String::new();
    let mut tempo = String::new();
    let mut maxima = String::new();
    let mut minima = String::new();
    let mut iuv = String::new();

    let mut n: Previsao;

    let mut coll: Vec<Previsao> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                ele = name.local_name;
            }
            Ok(XmlEvent::Characters(data)) => {
                if ele == "nome" {
                    nome = data.clone()
                }
                if ele == "uf" {
                    uf = data.clone()
                }
                if ele == "dia" {
                    dia = data.clone()
                }
                if ele == "tempo" {
                    tempo = data.clone()
                }
                if ele == "maxima" {
                    maxima = data.clone()
                }
                if ele == "maxima" {
                    minima = data.clone()
                }
                if ele == "iuv" {
                    iuv= data.clone()
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if "previsao" == name.local_name {
                    n = Previsao::new(dia.clone(), tempo.clone(), maxima.clone(), minima.clone(), iuv.clone());

                    coll.push(n.clone());
                }
            }
            Err(e) => {
                println!("{}", e);
            }

            _ => {}
        }
    }

    let mut table = String::new();
    for e in coll {
        table.push_str(&e.table_item())
    }

    content::Html(format!(r#"<html>
                <head>
                    <title>Rocket Tempo</title>
                </head>
                <body>
                    <h1>Previsao do tempo</h1>
                    <h3>{} - {}</h3>
                    <table border=1>
                        <tr>
                            <th>Dia</th>
                            <th>Tempo</th>
                            <th>Máxima</th>
                            <th>Mínima</th>
                            <th>IUV</th>
                        </tr>
                        {}
                    </table>
                </body>
            </html>"#, nome, uf, table)
    )
}

#[get("/pesquisa?<nome>")]
fn pesquisa(nome: &RawStr) -> content::Html<String> {
    let s = format!{"http://servicos.cptec.inpe.br/XML/listaCidades?city={}", nome.as_str()};
    let procura = reqwest::get(&s)
        .unwrap()
        .text()
        .unwrap();

    let parser = EventReader::from_str(&procura);
    let mut ele = String::new();
    let mut nome = String::new();
    let mut uf = String::new();
    let mut id = String::new();

    let mut n: Cidade;

    let mut coll: Vec<Cidade> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                ele = name.local_name;
            }
            Ok(XmlEvent::Characters(data)) => {
                if ele == "nome" {
                    nome = data.clone()
                }
                if ele == "uf" {
                    uf = data.clone()
                }
                if ele == "id" {
                    id = data.clone()
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if "cidade" == name.local_name {
                    n = Cidade::new(nome.clone(), uf.clone(), id.clone());

                    coll.push(n.clone());
                }
            }
            Err(e) => {
                println!("{}", e);
            }

            _ => {}
        }
    }

    let mut list = String::new();
    for e in coll {
        list.push_str(&e.list_item())
    }

    content::Html(format!(r#"<html>
                <head>
                    <title>Rocket Tempo</title>
                </head>
                <body>
                    <h1>Resultado da Pesquisa</h1>
                    <ul>
                        {}
                    </ul>
                </body>
            </html>"#, list)
    )
}
