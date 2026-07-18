# In-memory provider registry

Deterministic provider registry adapter used by application composition and tests. It keeps descriptors ordered by provider ID and rejects duplicate identity. It stores no secrets and grants no capability by registration alone.
