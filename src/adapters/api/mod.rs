/// All json objects that mail-tm API have
/// 
/// Warning! DateTime objects are not implemented yet!
pub(crate) mod schemas;

/// All mail-tm API errors 
pub(crate) mod errors;

/// All mail-tm API wrappers
/// 
/// Those will usually return structs from crate::adapters::api::schemas
pub(crate) mod functions;

#[cfg(test)]
mod tests;