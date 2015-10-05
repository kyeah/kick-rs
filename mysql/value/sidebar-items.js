initSidebarItems({"enum":[["Value","`Value` enumerates possible values in mysql cells. Also `Value` used to fill prepared statements."]],"fn":[["from_row","Will *panic* if could not convert `row` to `T`."],["from_row_opt","Will return `Err(row)` if could not convert `row` to `T`"],["from_value","Will panic if could not convert `v` to `T`"],["from_value_opt","Will return `Err(v)` if could not convert `v` to `T`"]],"struct":[["SignedDuration","A way to store negativeness of mysql's time. `.0 == true` means negative."]],"trait":[["FromRow","Trait to convert `Vec<Value>` into tuple of `FromValue` implementors up to arity 12."],["FromValue","Implement this trait to convert value to something."],["IntoValue","Implement this trait if you want to convert something to `Value`."],["ToRow",""],["ToValue",""]]});