/// All json objects that mail-tm API have
/// 
/// Warning! DateTime objects are not implemented yet!
pub(crate) mod schemas;

/// All mail-tm API errors 
pub(crate) mod errors;

/// All mail-tm API wrappers
pub(crate) mod functions;

#[cfg(test)]
mod tests;