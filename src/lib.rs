#[macro_export]
macro_rules! funkjon {
    (
        $ident:ident ::$(<
            $($generic:ident $(: $first_partial:ident
                $(:: $($first_residual:ident $(::)?)*)?
                $( $( +
                    $partial:ident
                    $(:: $($residual:ident $(::)?)*)?
                )+ )?
            )? ),*
        > ::)? $($param:ident),* $block:block
        as ( $($ty:ty),* ) -> Unit
        $(where $($clause_ident:ident :
            $first_clause_partial:ident
            $(:: $($first_clause_residual:ident $(::)?)*)?
            $( $(+
                $clause_partial:ident
                $(:: $($clause_residual:ident $(::)?)*)?
            )+ )?
        ),* $(,)?)?
    ) => {
        fn $ident
        $(< $($generic $(: $first_partial $($(:: $first_residual)*)? $($(+ $partial $($(:: $residual)*)?)*)?)?),* >)?
        ( $($param : $ty),* ) -> ()
            $(where $($clause_ident: $first_clause_partial $($(:: $first_clause_residual)*)? $($(+ $clause_partial $($($clause_residual)*)?)*)?),*)?
        $block
    };
    (
        $ident:ident ::$(<
            $($generic:ident $(: $first_partial:ident
                $(:: $($first_residual:ident $(::)?)*)?
                $( $( +
                    $partial:ident
                    $(:: $($residual:ident $(::)?)*)?
                )+ )?
            )? ),*
        > ::)? $($param:ident),* $block:block
        as ( $($ty:ty),* ) -> $output:ty
        $(where $($clause_ident:ident :
            $first_clause_partial:ident
            $(:: $($first_clause_residual:ident $(::)?)*)?
            $( $(+
                $clause_partial:ident
                $(:: $($clause_residual:ident $(::)?)*)?
            )+ )?
        ),* $(,)?)?
    ) => {
        fn $ident
        $(< $($generic $(: $first_partial $($(:: $first_residual)*)? $($(+ $partial $($(:: $residual)*)?)*)?)?),* >)?
        ( $($param : $ty),* ) -> $output
            $(where $($clause_ident: $first_clause_partial $($(:: $first_clause_residual)*)? $($(+ $clause_partial $($($clause_residual)*)?)*)?),*)?
        $block
    };
}
