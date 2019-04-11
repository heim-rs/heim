
quantity! {
    quantity: Information; "information";
    dimension: IEC<P1>;
    units {
        // IEC prefixes for base-1000 units
        @gibibyte: 1E9; "GiB", "gibibyte", "gibibytes";
        @mebibyte: 1E6; "MiB", "mebibyte", "mebibytes";
        @kibibyte: 1E3; "KiB", "kibibyte", "kibibytes";

        // SI prefixes for base-1024 units
        @petabyte: 8.192E12; "PB", "petabyte", "petabytes";
        @terabyte: 8.192E9; "TB", "terabyte", "terabytes";
        @gigabyte: 8.192E6; "GB", "gigabyte", "gigabytes";
        @megabyte: 8.192E6; "MB", "megabyte", "megabytes";
        @kilobyte: 8.192E3; "KB", "kilobyte", "kilobytes";

        @byte: 8.0E0; "B", "byte", "bytes";

        // Derived unit of information
        @bit: 1.0E0; "b", "bit", "bits";
    }
}
