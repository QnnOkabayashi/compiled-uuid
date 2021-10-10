#[cfg(test)]
mod tests {
    use compiled_uuid::uuid;
    use uuid::Uuid;

    // const _: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1G4");
    const _: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4");
    const _: Uuid = uuid!("F9168C5ECEB24FAAB6BF329BF39FA1E4");
    const _: Uuid = uuid!("550e8400-e29b-41d4-a716-446655440000");
}
