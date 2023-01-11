#[macro_export]
macro_rules! env_overwrite {
    ($field:expr, $name:expr) => {
        if let Some(var) = std::env::var($name).ok() {
            $field = var.into();
        }
    };
    ($field:expr, $name:expr, $parse:expr) => {
        if let Some(var) = std::env::var($name).ok() {
            $field = $parse(var);
        }
    };
}

#[macro_export]
macro_rules! merge_config_cli {
	($self:ident, $cli:ident, $field:ident, $default:expr, $parse:expr) => {
			match ($self.$field, $cli.$field) {
					(None, None) => $default,
					(None, Some(field))
					| (Some(field), None)
					// CLI option overrides
					| (Some(_), Some(field)) => $parse(field),
			}
	};

	($self:ident, $cli:ident, $field:ident, $default:expr) => {
		match ($self.$field, $cli.$field) {
				(None, None) => $default,
				(None, Some(field))
				| (Some(field), None)
				// CLI option overrides
				| (Some(_), Some(field)) => field,
		}
};
}
