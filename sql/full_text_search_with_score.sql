SELECT description, rating, category, paradedb.score(id)
FROM mock_items
WHERE description @@@ 'shoes' OR category @@@ 'footwear' AND rating @@@ '>2'
ORDER BY score DESC, description
LIMIT 5;
