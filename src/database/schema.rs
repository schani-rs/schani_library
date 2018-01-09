table! {
    images (id) {
        id -> Int4,
        raw_id -> Nullable<Bpchar>,
        image_id -> Nullable<Bpchar>,
        user_id -> Int4,
        sidecar_id -> Nullable<Bpchar>,
    }
}
