# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd glsl/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv glsl/fuzz/target/x86_64-unknown-linux-gnu/release/parse_expr /parse_expr && \
    mv glsl/fuzz/target/x86_64-unknown-linux-gnu/release/parse_integral_lit /parse_integral_lit && \
    mv glsl/fuzz/target/x86_64-unknown-linux-gnu/release/transpile_expr /transpile_expr && \
    mv glsl/fuzz/target/x86_64-unknown-linux-gnu/release/parse_translation_unit /parse_translation_unit && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd glsl/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv glsl/fuzz/target/release/parse_expr /parse_expr_no_inst && \
    mv glsl/fuzz/target/release/parse_integral_lit /parse_integral_lit_no_inst && \
    mv glsl/fuzz/target/release/transpile_expr /transpile_expr_no_inst && \
    mv glsl/fuzz/target/release/parse_translation_unit /parse_translation_unit_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /parse_expr /parse_expr_no_inst /
COPY --from=builder /parse_integral_lit /parse_integral_lit_no_inst /
COPY --from=builder /transpile_expr /transpile_expr_no_inst /
COPY --from=builder /parse_translation_unit /parse_translation_unit_no_inst /
