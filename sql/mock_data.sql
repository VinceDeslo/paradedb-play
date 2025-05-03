CALL paradedb.create_bm25_test_table(
  schema_name => 'public',
  table_name => 'mock_items'
);

SELECT description, rating, category
FROM mock_items
LIMIT 3;
