SELECT description, rating, category
FROM mock_items
WHERE description @@@ '"white shoes"~1'
LIMIT 5;
