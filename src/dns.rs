//! DNS.
//!
//! Objetivo de aprendizaje: entender resolución de nombres, autoridad, caché,
//! TTL y registros comunes.

use std::collections::BTreeMap;
use std::fmt;

/// Nombre de dominio normalizado para el modelo DNS educativo.
///
/// # Examples
///
/// ```
/// use rust_networking::dns::DomainName;
///
/// let name = DomainName::new("API.Jeresoft.Test.").unwrap();
/// assert_eq!(name.as_str(), "api.jeresoft.test");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainName(String);

impl DomainName {
    /// Crea y normaliza un nombre de dominio.
    ///
    /// Complejidad: O(n), donde `n` es la longitud del nombre.
    pub fn new(value: impl AsRef<str>) -> Result<Self, DnsError> {
        let normalized = value
            .as_ref()
            .trim()
            .trim_end_matches('.')
            .to_ascii_lowercase();

        if normalized.is_empty()
            || normalized.len() > 253
            || normalized
                .split('.')
                .any(|label| label.is_empty() || label.len() > 63)
        {
            return Err(DnsError::InvalidName {
                value: value.as_ref().to_string(),
            });
        }

        Ok(Self(normalized))
    }

    /// Devuelve el nombre normalizado.
    ///
    /// Complejidad: O(1).
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DomainName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Tipo de registro DNS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordType {
    /// Dirección IPv4.
    A,
    /// Dirección IPv6.
    Aaaa,
    /// Alias hacia otro nombre.
    Cname,
    /// Intercambiador de correo.
    Mx,
    /// Texto asociado al nombre.
    Txt,
}

/// Registro DNS educativo.
///
/// # Examples
///
/// ```
/// use rust_networking::dns::{DnsRecord, DomainName, RecordType};
///
/// let name = DomainName::new("api.jeresoft.test").unwrap();
/// let record = DnsRecord::a(name, "203.0.113.10", 300);
/// assert_eq!(record.record_type(), RecordType::A);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsRecord {
    name: DomainName,
    record_type: RecordType,
    value: String,
    ttl: u64,
}

impl DnsRecord {
    /// Crea un registro A.
    ///
    /// Complejidad: O(v), donde `v` es la longitud del valor.
    pub fn a(name: DomainName, address: impl Into<String>, ttl: u64) -> Self {
        Self::new(name, RecordType::A, address, ttl)
    }

    /// Crea un registro AAAA.
    ///
    /// Complejidad: O(v), donde `v` es la longitud del valor.
    pub fn aaaa(name: DomainName, address: impl Into<String>, ttl: u64) -> Self {
        Self::new(name, RecordType::Aaaa, address, ttl)
    }

    /// Crea un registro CNAME.
    ///
    /// Complejidad: O(v), donde `v` es la longitud del destino.
    pub fn cname(name: DomainName, target: DomainName, ttl: u64) -> Self {
        Self::new(name, RecordType::Cname, target.as_str(), ttl)
    }

    /// Crea un registro MX.
    ///
    /// Complejidad: O(v), donde `v` es la longitud del valor.
    pub fn mx(name: DomainName, exchange: impl Into<String>, ttl: u64) -> Self {
        Self::new(name, RecordType::Mx, exchange, ttl)
    }

    /// Crea un registro TXT.
    ///
    /// Complejidad: O(v), donde `v` es la longitud del valor.
    pub fn txt(name: DomainName, text: impl Into<String>, ttl: u64) -> Self {
        Self::new(name, RecordType::Txt, text, ttl)
    }

    fn new(name: DomainName, record_type: RecordType, value: impl Into<String>, ttl: u64) -> Self {
        Self {
            name,
            record_type,
            value: value.into(),
            ttl,
        }
    }

    /// Devuelve el nombre del registro.
    ///
    /// Complejidad: O(1).
    pub fn name(&self) -> &DomainName {
        &self.name
    }

    /// Devuelve el tipo del registro.
    ///
    /// Complejidad: O(1).
    pub fn record_type(&self) -> RecordType {
        self.record_type
    }

    /// Devuelve el valor textual del registro.
    ///
    /// Complejidad: O(1).
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Devuelve el TTL en segundos.
    ///
    /// Complejidad: O(1).
    pub fn ttl(&self) -> u64 {
        self.ttl
    }
}

/// Zona autoritativa educativa.
#[derive(Debug, Clone, Default)]
pub struct Zone {
    records: BTreeMap<DomainName, Vec<DnsRecord>>,
}

impl Zone {
    /// Crea una zona vacía.
    ///
    /// Complejidad: O(1).
    pub fn new() -> Self {
        Self::default()
    }

    /// Agrega un registro a la zona.
    ///
    /// Complejidad: O(log n).
    pub fn add_record(&mut self, record: DnsRecord) {
        self.records
            .entry(record.name().clone())
            .or_default()
            .push(record);
    }

    fn records_for(&self, name: &DomainName) -> Option<&[DnsRecord]> {
        self.records.get(name).map(Vec::as_slice)
    }
}

/// Resultado de resolución DNS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    canonical_name: DomainName,
    records: Vec<DnsRecord>,
    from_cache: bool,
    expires_at: u64,
}

impl Resolution {
    fn new(
        canonical_name: DomainName,
        records: Vec<DnsRecord>,
        from_cache: bool,
        now: u64,
    ) -> Self {
        let ttl = records.iter().map(DnsRecord::ttl).min().unwrap_or(0);
        Self {
            canonical_name,
            records,
            from_cache,
            expires_at: now.saturating_add(ttl),
        }
    }

    /// Devuelve el nombre canónico resuelto.
    ///
    /// Complejidad: O(1).
    pub fn canonical_name(&self) -> &DomainName {
        &self.canonical_name
    }

    /// Devuelve registros finales de la resolución.
    ///
    /// Complejidad: O(1).
    pub fn records(&self) -> &[DnsRecord] {
        &self.records
    }

    /// Indica si la respuesta salió de caché.
    ///
    /// Complejidad: O(1).
    pub fn from_cache(&self) -> bool {
        self.from_cache
    }

    /// Devuelve el instante educativo de expiración.
    ///
    /// Complejidad: O(1).
    pub fn expires_at(&self) -> u64 {
        self.expires_at
    }

    fn as_cached(&self) -> Self {
        let mut cached = self.clone();
        cached.from_cache = true;
        cached
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CacheKey {
    name: DomainName,
    record_type: RecordType,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    resolution: Resolution,
}

/// Resolvedor DNS educativo con caché inyectada por tiempo lógico.
#[derive(Debug, Clone)]
pub struct Resolver {
    zone: Zone,
    cache: BTreeMap<CacheKey, CacheEntry>,
    cname_limit: usize,
}

impl Resolver {
    /// Crea un resolvedor con límite de CNAME por defecto.
    ///
    /// Complejidad: O(1).
    pub fn new(zone: Zone) -> Self {
        Self {
            zone,
            cache: BTreeMap::new(),
            cname_limit: 8,
        }
    }

    /// Ajusta el límite de saltos CNAME.
    ///
    /// Complejidad: O(1).
    pub fn with_cname_limit(mut self, limit: usize) -> Self {
        self.cname_limit = limit;
        self
    }

    /// Resuelve un nombre y tipo usando un reloj lógico inyectado.
    ///
    /// Complejidad: O(c * r log n), donde `c` son saltos CNAME y `r` registros
    /// por nombre.
    pub fn resolve(
        &mut self,
        name: &DomainName,
        record_type: RecordType,
        now: u64,
    ) -> Result<Resolution, DnsError> {
        let key = CacheKey {
            name: name.clone(),
            record_type,
        };

        if let Some(entry) = self.cache.get(&key) {
            if now < entry.resolution.expires_at() {
                return Ok(entry.resolution.as_cached());
            }
        }

        let resolution = self.resolve_uncached(name, record_type, now, 0)?;
        self.cache.insert(
            key,
            CacheEntry {
                resolution: resolution.clone(),
            },
        );
        Ok(resolution)
    }

    fn resolve_uncached(
        &self,
        name: &DomainName,
        record_type: RecordType,
        now: u64,
        depth: usize,
    ) -> Result<Resolution, DnsError> {
        let records = self
            .zone
            .records_for(name)
            .ok_or_else(|| DnsError::NxDomain { name: name.clone() })?;

        let matching_records = records
            .iter()
            .filter(|record| record.record_type() == record_type)
            .cloned()
            .collect::<Vec<_>>();

        if !matching_records.is_empty() {
            return Ok(Resolution::new(name.clone(), matching_records, false, now));
        }

        let cname = records
            .iter()
            .find(|record| record.record_type() == RecordType::Cname);

        if let Some(cname) = cname {
            if depth >= self.cname_limit {
                return Err(DnsError::CnameLimitExceeded {
                    name: name.clone(),
                    limit: self.cname_limit,
                });
            }

            let target = DomainName::new(cname.value())?;
            return self.resolve_uncached(&target, record_type, now, depth + 1);
        }

        Err(DnsError::NoRecords {
            name: name.clone(),
            record_type,
        })
    }
}

/// Error educativo del resolvedor DNS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsError {
    /// El nombre de dominio no es válido.
    InvalidName { value: String },
    /// El nombre no existe en la zona.
    NxDomain { name: DomainName },
    /// El nombre existe, pero no tiene registros del tipo solicitado.
    NoRecords {
        name: DomainName,
        record_type: RecordType,
    },
    /// La cadena CNAME excedió el límite educativo.
    CnameLimitExceeded { name: DomainName, limit: usize },
}
