#![allow(unused_imports)]

// Agregar los repositorios aquí
pub mod album_repository;
pub mod artista_repository;
pub mod cancion_repository;
pub mod usuario_streaming_repository;
pub mod playlist_repository;
pub use album_repository::AlbumRepository;
pub use artista_repository::ArtistaRepository;
pub use cancion_repository::CancionRepository;
pub use usuario_streaming_repository::UsuarioStreamingRepository;
pub use playlist_repository::PlaylistRepository;
