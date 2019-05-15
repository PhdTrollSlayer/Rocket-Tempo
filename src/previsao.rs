use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Previsao {
    dia: String,
    tempo: String,
    maxima: String,
    minima: String,
    iuv: String,
}

impl Previsao {
    pub fn new(dia: String, tempo: String, maxima: String, minima: String, iuv: String) -> Previsao {
        let mut hm: HashMap<&str, &str> = HashMap::new();
        hm.insert("ec","Encoberto com Chuvas Isoladas");
        hm.insert("ci","Chuvas Isoladas");
        hm.insert("c","Chuva");
        hm.insert("in","Instável");
        hm.insert("pp","Poss. de Pancadas de Chuva");
        hm.insert("cm","Chuva pela Manhã");
        hm.insert("cn","Chuva a Noite");
        hm.insert("pt","Pancadas de Chuva a Tarde");
        hm.insert("pm","Pancadas de Chuva pela Manhã");
        hm.insert("np","Nublado e Pancadas de Chuva");
        hm.insert("pc","Pancadas de Chuva");
        hm.insert("pn","Parcialmente Nublado");
        hm.insert("cv","Chuvisco");
        hm.insert("ch","Chuvoso");
        hm.insert("t","Tempestade");
        hm.insert("ps","Predomínio de Sol");
        hm.insert("e","Encoberto");
        hm.insert("n","Nublado");
        hm.insert("cl","Céu Claro");
        hm.insert("nv","Nevoeiro");
        hm.insert("g","Geada");
        hm.insert("ne","Neve");
        hm.insert("nd","Não Definido");
        hm.insert("pnt","Pancadas de Chuva a Noite");
        hm.insert("psc","Possibilidade de Chuva");
        hm.insert("pcm","Possibilidade de Chuva pela Manhã");
        hm.insert("pct","Possibilidade de Chuva a Tarde");
        hm.insert("npt","Possibilidade de Chuva a Noite");
        hm.insert("npn","Nublado com Pancadas a Noite");
        hm.insert("ncn","Nublado com Poss. de Chuva a Noite");
        hm.insert("nct","Nublado com Poss. de Chuva a Tarde");
        hm.insert("ncm","Nubl. c/ Poss. de Chuva pela Manhã");
        hm.insert("npm","Nublado com Pancadas pela Manhã");
        hm.insert("npp","Nublado com Possibilidade de Chuva");
        hm.insert("vn","Variação de Nebulosidade");
        hm.insert("ct","Chuva a Tarde");
        hm.insert("ppn","Poss. de Panc. de Chuva a Noite");
        hm.insert("ppt","Poss. de Panc. de Chuva a Tarde");
        hm.insert("ppm","Poss. de Panc. de Chuva pela Manhã");

        Previsao {
            dia,
            tempo: hm.get::<str>(&tempo).unwrap().to_string(),
            maxima: format!("{} °C", maxima),
            minima: format!("{} °C", minima),
            iuv,
        }
    }

    pub fn table_item(&self) -> String {
        format!("<tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>", self.dia, self.tempo, self.maxima, self.minima, self.iuv)
    }
}
