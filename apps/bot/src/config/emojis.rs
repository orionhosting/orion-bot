use twilight_model::id::{Id, marker::EmojiMarker};

/// A macro to generate the development & production emojis.
macro_rules! emojis {
    (
        $(
            $name:ident => ($debug_id:literal, $release_id:literal), $kind:ident;
        )*
    ) => {
        $(
            #[cfg(debug_assertions)]
            paste::paste! {
                pub const $name: &'static str = concat!(
                    emojis!(@prefix $kind),
                    "_:",
                    $debug_id,
                    ">"
                );

                #[allow(dead_code)]
                pub const [<$name _ID>]: u64 = $debug_id;
                #[allow(dead_code)]
                pub const [<$name _EID>]: Id<EmojiMarker> = Id::new($debug_id);
            }

            #[cfg(not(debug_assertions))]
            paste::paste! {
                pub const $name: &'static str = concat!(
                    emojis!(@prefix $kind),
                    "_:",
                    $release_id,
                    ">"
                );

                pub const [<$name _ID>]: u64 = $release_id;
                pub const [<$name _EID>]: Id<EmojiMarker> = Id::new($release_id);
            }
        )*
    };

    (@prefix animated) => { "<a:" };
    (@prefix static) => { "<:" };
}

/// The bot emojis.
pub struct Emojis;

#[allow(dead_code)]
impl Emojis {
    pub const WARN: &'static str = ":x:";
    pub const SUCCESS: &'static str = ":white_check_mark:";
}

impl Emojis {
    emojis! {
        // first ID is debug, second ID is release
        PROPERTY     => (1477495809418461295, 1479652366092599456), static;
        LOADER_GREEN => (1478113950964125827, 1479652276573438003), animated;
        LOGO         => (1480023836442492978, 1480023652463542322), static;
        DISCORD      => (1480032584540094607, 1481776188329427025), static;
        ONLINE       => (1481020115179798698, 1481041163074011380), static;
        DND          => (1481020113351213077, 1481041161652277494), static;
    }
}
