# Terminology Note

This code predates v0.17. It uses "domain" to mean "enum only."

In v0.17, **domain** means any data definition (enum + struct +
newtype). The `()` form is called **enum**, not "domain."

- `Domain` in this code → `Enum` in v0.17
- `DomainKind` → there is no equivalent; domain is the concept,
  enum/struct/newtype are the forms
- `DialectKind::Domain` → `DialectKind::Enum`
- `NodeKind::Domain` → `NodeKind::Enum`
- `ScopeKind::Domain` → `ScopeKind::Enum`

Do not carry this old terminology into new code.
