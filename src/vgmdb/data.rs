#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct Album<T> {
      pub release_date: String
    , pub discs: Vec<Disc<T>>
    , pub catalog: Option<String>
}

#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct Disc<T>{
      pub disc_length: String
    , pub name: String
    , pub tracks: Vec<T>
}

#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct TrackDb{
      pub names: Names
    , pub track_length: String
}

#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct Track{
      pub name : String
    , pub track_length : i32
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable, Debug )]
pub struct Names{
      pub English:  Option<String>
    , pub Romaji:   Option<String>
    , pub Japanese: Option<String>
}
