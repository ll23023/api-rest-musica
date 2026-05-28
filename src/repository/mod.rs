// Agregar los repositorios aquí
pub mod album_repository;
pub mod artista_repository;
pub mod cancion_repository;
pub mod usuariostreaming_repository;

pub use artista_repository::ArtistaRepository;
pub use cancion_repository::CancionRepository;
pub use album_repository::AlbumRepository;
pub use usuariostreaming_repository::UsuarioStreamingRepository;