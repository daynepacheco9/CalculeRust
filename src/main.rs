struct Sala {
    largura: f64,
    profundidade: f64,
    altura: f64,
}

impl Sala {

    fn area_piso(&self) -> f64 {
        self.largura * self.profundidade
    }


    fn area_parede_frontal(&self) -> f64 {
        self.largura * self.altura
    }


    fn area_parede_lateral(&self) -> f64 {
        self.profundidade * self.altura
    }


    fn volume(&self) -> f64 {
        self.largura * self.profundidade * self.altura
    }


    fn soma_volumes(&self, outra_sala: &Sala) -> f64 {
        self.volume() + outra_sala.volume()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_piso() {
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.area_piso(), 20.0);
    }

    #[test]
    fn test_area_parede_frontal() {
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.area_parede_frontal(), 15.0);
    }

    #[test]
    fn test_area_parede_lateral() {
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.area_parede_lateral(), 12.0);
    }

    #[test]
    fn test_volume() {
        let sala = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        assert_eq!(sala.volume(), 60.0);
    }

    #[test]
    fn test_soma_volumes() {
        let sala1 = Sala { largura: 5.0, profundidade: 4.0, altura: 3.0 };
        let sala2 = Sala { largura: 3.0, profundidade: 2.0, altura: 2.5 };
        assert_eq!(sala1.soma_volumes(&sala2), 72.5);
    }
}
fn main() {

    let sala = Sala {
        largura: 5.0,
        profundidade: 4.0,
        altura: 3.0,
    };

    let sala2 = Sala {
        largura: 3.0,
        profundidade: 2.0,
        altura: 2.5,
    };
    
    println!("Área do piso: {}", sala.area_piso());
    println!("Área da parede frontal: {}", sala.area_parede_frontal());
    println!("Área da parede lateral: {}", sala.area_parede_lateral());
    println!("Volume: {}", sala.volume());
    println!("Volume da sala 2: {}", sala2.volume());
    println!("Soma dos volumes: {}", sala.soma_volumes(&sala2));
}
