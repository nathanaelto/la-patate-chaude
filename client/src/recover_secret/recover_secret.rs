use common::challenge::Challenge;

pub struct RecoverSecretInput {
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

pub struct RecoverSecret {
    pub input: RecoverSecretInput,
    pub output: RecoverSecretOutput,
}

impl Challenge for RecoverSecret {
    type Input = RecoverSecretInput;
    type Output = RecoverSecretOutput;
    fn name() -> String {
        String::from("Recover secret")
    }

    fn new(input: Self::Input) -> Self {
        // récupère l'input et le mettre dans un format travaillable
        let letters: Vec<char> = input.letters.chars().collect();
        let size: usize = input.tuple_sizes.len();
        let mut start_index = 0;

        // boucle sur tuple_sizes.size et on forme les uplets dans la boucle
        for n in input.tuple_sizes.iter() {
            let size = n;
            let mut uplet =vec![&letters.chunks(*n); *n];
            // faire un vec d'uplet ?

            start_index = n.clone();
        }
        todo!()
    }

    fn solve(&self) -> Self::Output {
        //vérifier avant de solver?


        todo!()
    }

    fn verify(&self, answer: Self::Output) -> bool {
        todo!()
    }
}