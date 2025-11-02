#[macro_export]
macro_rules! bidi_command {
    ($command_base: expr, $command: expr, $method: expr, $params: ident $params_block: block) => {
        CommandData::$command_base($command_base::$command(
                $command {
                    method: $method,
                    params: $params $params_block
                },
            ))
    };
}