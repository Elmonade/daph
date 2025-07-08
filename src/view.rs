/*
pub fn setup() -> (mpsc::Sender<Command>, mpsc::Receiver<SinkState>) {
    let (command_tx, command_rx) = mpsc::channel::<Command>();
    let (state_tx, state_rx) = mpsc::channel::<SinkState>();

    let _ = thread::Builder::new()
        .name("playback".to_string())
        .spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            loop {
                if let Ok(command) = command_rx.try_recv() {
                    audio_command(command, &sink);
                }

                //TODO: Not a good idea to recreate this variable every 100ms.
                let sink_state = SinkState {
                    que_len: sink.len(),
                    is_paused: sink.is_paused(),
                    is_empty: sink.empty(),
                };

                state_tx.send(sink_state).unwrap_or(());

                thread::sleep(time::Duration::from_millis(100));
            }
        });
    (command_tx, state_rx)
}
*/
