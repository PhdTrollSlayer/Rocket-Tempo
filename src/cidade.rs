#[derive(Debug, Clone)]
pub struct Cidade {
    pub nome: String,
    pub uf: String,
    pub id: String,
    pub url: String,
}

impl Cidade {
    pub fn new(nome: String, uf: String, id: String) -> Cidade {
        let url = format!("/previsao?id={}", id.clone());
        Cidade {
            nome,
            uf,
            id,
            url,
        }
    }
    pub fn list_item(&self) -> String {
        format!("<a href={}><li>{} - {}</li></a>\n", self.url, self.nome, self.uf)
    }
}
