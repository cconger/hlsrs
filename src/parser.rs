use std::str;

pub enum Playlist {
    Multivariant(MultivariantPlaylist),
    Media(MediaPlaylist),
}

#[derive(Debug, PartialEq)]
pub struct MediaPlaylist {
    // EXT-X-VERSION
    pub version: Option<u32>,
    // EXT-X-INDEPENDENT-SEGMENTS
    pub independent_segments: bool,
    // EXT-X-START
    pub start: Option<Start>,
    // EXT-X-DEFINE
    pub variables: Vec<Variable>,
    // EXT-X-TARGETDURATION
    pub target_duration: u64,
    // EXT-X-MEDIA-SEQUENCE
    pub media_sequence: u64,
    // EXT-X-DISCONTINUITY-SEQUENCE
    pub discontinuity_sequence: u64,
    // EXT-X-ENDLIST
    pub end_list: bool,
    // EXT-x-PLAYLIST-TYPE
    pub playlist_type: Option<MediaPlaylistType>,
    // EXT-X-I-FRAMES-ONLY
    pub i_frames_only: bool,
    // EXT-X-PART-INF
    pub part_target: Option<f64>,
    // EXT-X-SERVER-CONTROL
    pub server_control: Option<ServerControl>,

    pub segments: Vec<MediaSegment>,

    pub skip: Option<Skip>,
    pub dateranges: Option<Vec<String>>,    // TODO: Better type
    pub preload_hints: Option<Vec<String>>, // TODO: Better type
    pub rendition_reports: Option<Vec<String>>, // TODO: Better type
}

#[derive(Debug, PartialEq)]
pub struct MediaSegment {
    pub uri: String,
    pub mime_type: Option<String>,
    pub duration: f64,
    pub title: Option<String>,
    pub byte_range: Option<ByteRange>,
    pub discontinuity: bool,
    pub key: Option<Key>,
    pub map: Option<Map>,
    pub program_date_time: Option<String>, // TODO; Date?
    pub gap: bool,
    pub bitrate: Option<u64>,
    pub parts: Vec<PartialSegment>,
    pub extra_tags: Vec<ExtTag>,
}

#[derive(Debug, PartialEq)]
pub struct ExtTag {
    pub tag_name: String,
    pub tag_payload: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct PartialSegment {
    pub uri: String,
    pub duration: f64,
    pub indepdentent: bool,
    pub byte_range: ByteRange,
    pub gap: bool,
}

#[derive(Debug, PartialEq)]
pub struct Key {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct Map {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct Skip {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct ByteRange {
    pub length: u64,
    pub offset: Option<u64>,
}

#[derive(Debug, PartialEq)]
pub enum Variable {
    Name((String, String)),
    Import(String),
}

#[derive(Debug, PartialEq)]
pub enum MediaPlaylistType {
    Event,
    Vod,
}

#[derive(Debug, PartialEq)]
pub struct Start {
    // TIME-OFFSET Attr
    pub time_offset: f64,
    // PRECISE attr
    pub precise: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub struct ServerControl {
    // TODO
}

#[derive(Debug, PartialEq)]
pub struct MultivariantPlaylist {}

pub fn parse(input: &[u8]) -> Result<Playlist, String> {
    let instr = match str::from_utf8(input) {
        Ok(v) => v,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    // Create tags from input
    let tags: Vec<Tag> = instr.lines().filter_map(|l| parse_tag(l).ok()).collect();

    if is_multivariant_manifest(&tags) {
        return Ok(Playlist::Multivariant(multivariant_from_tags(&tags)?));
    }
    return Ok(Playlist::Media(media_from_tags(&tags)?));
}

fn multivariant_from_tags(_tags: &Vec<Tag>) -> Result<MultivariantPlaylist, String> {
    Err("not implemented".to_string())
}

fn media_from_tags(tags: &Vec<Tag>) -> Result<MediaPlaylist, String> {
    let mut version = None;
    let independent_segments = false;
    let start = None;
    let variables = vec![];
    let mut target_duration = 0;
    let media_sequence = 0;
    let discontinuity_sequence = 0;
    let mut end_list = false;
    let playlist_type = None;
    let i_frames_only = false;
    let part_target = None;
    let server_control = None;
    let mut segments = vec![];
    let skip = None;
    let dateranges = None;
    let preload_hints = None;
    let rendition_reports = None;

    let mut segment = MediaSegment {
        uri: "".to_string(),
        mime_type: None,
        duration: 0.0,
        title: None,
        byte_range: None,
        discontinuity: false,
        key: None,
        map: None,
        program_date_time: None,
        gap: false,
        bitrate: None,
        parts: vec![],
        extra_tags: vec![],
    };
    for t in tags {
        match t {
            Tag::Version(v) => version = Some(*v),
            Tag::TargetDuration(d) => target_duration = *d,
            Tag::INF(d, t) => {
                segment.duration = *d;
                segment.title = t.clone();
            }
            Tag::Uri(u) => {
                segment.uri = u.to_string();
                segments.push(segment);
                segment = MediaSegment {
                    uri: "".to_string(),
                    mime_type: None,
                    duration: 0.0,
                    title: None,
                    byte_range: None,
                    discontinuity: false,
                    key: None,
                    map: None,
                    program_date_time: None,
                    gap: false,
                    bitrate: None,
                    parts: vec![],
                    extra_tags: vec![],
                };
            }
            Tag::EndList => {
                end_list = true;
            }
            _ => {}
        }
    }

    Ok(MediaPlaylist {
        version,
        independent_segments,
        start,
        variables,
        target_duration,
        media_sequence,
        discontinuity_sequence,
        end_list,
        playlist_type,
        i_frames_only,
        part_target,
        server_control,
        segments,
        skip,
        dateranges,
        preload_hints,
        rendition_reports,
    })
}

fn is_multivariant_manifest(tags: &Vec<Tag>) -> bool {
    for t in tags {
        match t {
            Tag::TargetDuration(_) | Tag::PlaylistType(_) => {
                return false;
            }
            _ => (),
        }
    }

    return false;
}

fn no_empty_str(t: &str) -> Option<String> {
    return if t.len() > 0 {
        Some(t.to_string())
    } else {
        None
    };
}

// TODO: Consider an alternative where all Tags instead of copying strings are holding references
// into the original, by wrapping &str.  Wouldn't that be neat....
fn parse_tag(line: &str) -> Result<Tag, String> {
    if !line.starts_with('#') {
        // URI
        return Ok(Tag::Uri(line.to_string()));
    }

    let (tag, rest) = match line.split_once(':') {
        None => (line.strip_prefix("#"), ""),
        Some((t, rest)) => (t.strip_prefix("#"), rest),
    };

    return match tag {
        Some("EXTM3U") => Ok(Tag::EXTM3U),
        Some("EXT-X-VERSION") => Ok(Tag::Version(rest.parse().unwrap())),
        Some("EXT-X-TARGETDURATION") => Ok(Tag::TargetDuration(rest.parse().unwrap())),
        Some("EXTINF") => {
            return match rest.split_once(',') {
                Some((d, t)) => Ok(Tag::INF(d.parse().unwrap(), no_empty_str(t))),
                None => Ok(Tag::INF(rest.parse().unwrap(), None)),
            }
        }
        Some("EXT-X-PLAYLIST-TYPE") => match rest {
            "VOD" => Ok(Tag::PlaylistType(MediaPlaylistType::Vod)),
            "EVENT" => Ok(Tag::PlaylistType(MediaPlaylistType::Event)),
            _ => Err("unsupported playlist type".to_string()),
        },
        Some("EXT-X-ENDLIST") => Ok(Tag::EndList),
        Some(t) => Ok(Tag::UnknownTag(t.to_string(), no_empty_str(rest))),
        None => Err("no tag".to_string()),
    };
}

#[derive(Debug, PartialEq)]
enum Tag {
    EXTM3U,
    TargetDuration(u64),
    PlaylistType(MediaPlaylistType),
    Version(u32),
    INF(f64, Option<String>),

    // Not really tags
    Uri(String),
    Comment(String),
    UnknownTag(String, Option<String>),
    EndList,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ext3mu() {
        let res = parse_tag("#EXTM3U").unwrap();
        assert_eq!(Tag::EXTM3U, res)
    }

    #[test]
    fn parse_targetduration() {
        let res = parse_tag("#EXT-X-TARGETDURATION:10").unwrap();
        assert_eq!(Tag::TargetDuration(10), res)
    }

    #[test]
    fn parse_version() {
        let res = parse_tag("#EXT-X-VERSION:3").unwrap();
        assert_eq!(Tag::Version(3), res)
    }

    #[test]
    fn parse_extinf() {
        let res = parse_tag("#EXTINF:9.009,").unwrap();
        assert_eq!(Tag::INF(9.009, None), res)
    }
    
    #[test]
    fn parse_playlisttype() {
        let res = parse_tag("#EXT-X-PLAYLIST-TYPE:VOD").unwrap();
        assert_eq!(Tag::PlaylistType(MediaPlaylistType::Vod), res)
    }

    #[test]
    fn parse_uri() {
        let res = parse_tag("https://chunk-gce-us-west1.fastly.staging.mux.com/v1/chunk/n5HuqAuIlDwB8sadK00kuV02UcHXJZCsOtwmFGnkWXfoCYQrbRvdJcY2I013UZzdknO01aoIkFE02sfzi01KIpcCxaRm21gav97wlpcvAj4DD00mww/0.ts?skid=default&signature=NjJhMTI5ZjBfZDQ5NTcyNjUwMzQyYTI3MzFiMjU5YjlhZjg2MGVhMTQzMjQ5NDY5YjRjMDhhNDVhNmRmY2IwMzRhZTRiNGJiNQ==").unwrap();
        assert_eq!(Tag::Uri("https://chunk-gce-us-west1.fastly.staging.mux.com/v1/chunk/n5HuqAuIlDwB8sadK00kuV02UcHXJZCsOtwmFGnkWXfoCYQrbRvdJcY2I013UZzdknO01aoIkFE02sfzi01KIpcCxaRm21gav97wlpcvAj4DD00mww/0.ts?skid=default&signature=NjJhMTI5ZjBfZDQ5NTcyNjUwMzQyYTI3MzFiMjU5YjlhZjg2MGVhMTQzMjQ5NDY5YjRjMDhhNDVhNmRmY2IwMzRhZTRiNGJiNQ==".to_string()), res)
    }

    #[test]
    fn test_parse_tags() {
        let expected = vec![
            Tag::EXTM3U,
            Tag::TargetDuration(10),
            Tag::Version(3),
            Tag::INF(9.009, None),
            Tag::Uri("http://media.example.com/first.ts".to_string()),
            Tag::INF(9.009, None),
            Tag::Uri("http://media.example.com/second.ts".to_string()),
            Tag::INF(3.003, None),
            Tag::Uri("http://media.example.com/third.ts".to_string()),
            Tag::EndList,
        ];

        let input = include_str!("../fixtures/simple.m3u8");
        let tags: Vec<Tag> = input.lines().filter_map(|l| parse_tag(l).ok()).collect();
        assert_eq!(expected, tags)
    }

    #[test]
    fn parse_simple() {
        let expected = MediaPlaylist{
            version: Some(3),
            independent_segments: false,
            start: None,
            variables: vec![],
            target_duration:10,
            media_sequence: 0,
            discontinuity_sequence: 0,
            end_list: true,
            playlist_type: None,
            i_frames_only: false,
            part_target: None,
            server_control: None,
            skip: None,
            dateranges: None,
            preload_hints: None,
            rendition_reports: None,
            segments: vec![
                MediaSegment{
                    uri: "http://media.example.com/first.ts".to_string(),
                    title: None,
                    mime_type: None,
                    duration: 9.009,
                    byte_range: None,
                    discontinuity: false,
                    key: None,
                    map: None,
                    program_date_time: None,
                    gap: false,
                    bitrate: None,
                    parts: vec![],
                    extra_tags: vec![],
                },
                MediaSegment{
                    uri: "http://media.example.com/second.ts".to_string(),
                    title: None,
                    mime_type: None,
                    duration: 9.009,
                    byte_range: None,
                    discontinuity: false,
                    key: None,
                    map: None,
                    program_date_time: None,
                    gap: false,
                    bitrate: None,
                    parts: vec![],
                    extra_tags: vec![],
                },
                MediaSegment{
                    uri: "http://media.example.com/third.ts".to_string(),
                    title: None,
                    mime_type: None,
                    duration: 3.003,
                    byte_range: None,
                    discontinuity: false,
                    key: None,
                    map: None,
                    program_date_time: None,
                    gap: false,
                    bitrate: None,
                    parts: vec![],
                    extra_tags: vec![],
                },
            ],
        };

        let input = include_str!("../fixtures/simple.m3u8");
        match parse(input.as_bytes()) {
            Ok(Playlist::Multivariant(_)) => (),
            Ok(Playlist::Media(pl)) => {assert_eq!(expected, pl)},
            Err(e) => {assert_eq!(e, "unimplement") },
        }
    }
}
