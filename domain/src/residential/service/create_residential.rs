struct CreateResidentialService<T: ResidentialRepository> {
    residential_repo: T,
}

impl<T: ResidentialRepository> CreateResidentialService<T> {
    pub fn new(residential_repo: T) -> Self {
        Self { residential_repo }
    }
}
