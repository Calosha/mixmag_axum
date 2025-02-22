use diesel::allow_tables_to_appear_in_same_query;

allow_tables_to_appear_in_same_query!(cd_review, name);

diesel::table! {
    article (__id) {
        __id -> Unsigned<Integer>,
        __status -> Unsigned<TinyInt>,
        __created -> Unsigned<Integer>,
        __updated -> Unsigned<Integer>,
        subTitle -> Nullable<Varchar>,
        imgWide -> Nullable<Varchar>,
        imgTop -> Nullable<Varchar>,
        isMain -> Nullable<Unsigned<TinyInt>>,
        announcementMain -> Nullable<Varchar>,
        imgMain -> Nullable<Varchar>,
        isBlack -> Nullable<Unsigned<TinyInt>>,
        ljId -> Nullable<Unsigned<Integer>>,
        title -> Varchar,
        announcement -> Nullable<Mediumtext>,
        description -> Nullable<Mediumtext>,
        tags -> Nullable<Binary>,
        uid -> Nullable<Unsigned<Integer>>,
        views -> Nullable<Unsigned<Integer>>,
        hash -> Nullable<Varchar>,
        commentAccess -> Nullable<Unsigned<TinyInt>>,
        ccount -> Nullable<Unsigned<Integer>>,
        pubDate -> Nullable<Unsigned<Integer>>,
        rate -> Nullable<Unsigned<Integer>>,
        ratePositive -> Nullable<Unsigned<Integer>>,
        rateNegative -> Nullable<Unsigned<Integer>>,
        ratedList -> Nullable<Mediumblob>,
    }
}
diesel::table! {
    news (__id) {
        __id -> Unsigned<Integer>,
        __status -> Unsigned<TinyInt>,
        __created -> Unsigned<Integer>,
        __updated -> Unsigned<Integer>,
        podcastId -> Nullable<Unsigned<Integer>>,
        subTitle -> Nullable<Varchar>,
        imgWide -> Nullable<Varchar>,
        imgTop -> Nullable<Varchar>,
        isMain -> Nullable<Unsigned<TinyInt>>,
        announcementMain -> Nullable<Varchar>,
        imgMain -> Nullable<Varchar>,
        isBlack -> Nullable<Unsigned<TinyInt>>,
        ljId -> Nullable<Unsigned<Integer>>,
        title -> Varchar,
        announcement -> Nullable<Mediumtext>,
        description -> Nullable<Mediumtext>,
        tags -> Nullable<Binary>,
        uid -> Nullable<Unsigned<Integer>>,
        views -> Nullable<Unsigned<Integer>>,
        hash -> Nullable<Varchar>,
        commentAccess -> Nullable<Unsigned<TinyInt>>,
        ccount -> Nullable<Unsigned<Integer>>,
        pubDate -> Nullable<Unsigned<Integer>>,
        rate -> Nullable<Unsigned<Integer>>,
        ratePositive -> Nullable<Unsigned<Integer>>,
        rateNegative -> Nullable<Unsigned<Integer>>,
        ratedList -> Nullable<Mediumblob>,
    }
}
diesel::table! {
    cd_review (__id) {
        __id -> Unsigned<Integer>,
        __status -> Unsigned<TinyInt>,
        __created -> Unsigned<Integer>,
        __updated -> Unsigned<Integer>,
        nameId -> Nullable<Unsigned<Integer>>,
        title -> Nullable<Varchar>,
        labelList -> Nullable<Binary>,
        genreList -> Nullable<Binary>,
        tags -> Nullable<Binary>,
        year -> Nullable<SmallInt>,
        trackListUrl -> Nullable<Varchar>,
        trackList -> Nullable<Mediumblob>,
        description -> Nullable<Mediumtext>,
        imgBig -> Nullable<Varchar>,
        uid -> Nullable<Unsigned<Integer>>,
        views -> Nullable<Unsigned<Integer>>,
        hash -> Nullable<Varchar>,
        ccount -> Nullable<Unsigned<Integer>>,
        pubDate -> Nullable<Unsigned<Integer>>,
        rate -> Nullable<Unsigned<Integer>>,
        ratePositive -> Nullable<Unsigned<Integer>>,
        rateNegative -> Nullable<Unsigned<Integer>>,
        ratedList -> Nullable<Mediumblob>,
    }
}
diesel::table! {
    name (__id) {
        __id -> Unsigned<Integer>,
        __status -> Unsigned<TinyInt>,
        __created -> Unsigned<Integer>,
        __updated -> Unsigned<Integer>,
        ruName -> Nullable<Varchar>,
        #[sql_name = "name"]
        name_col -> Nullable<Varchar>,
        pubDate -> Unsigned<Integer>,
        rate -> Unsigned<Integer>,
        ratePositive -> Unsigned<Integer>,
        rateNegative -> Unsigned<Integer>,
        ratedList -> Nullable<Mediumblob>,
    }
}
