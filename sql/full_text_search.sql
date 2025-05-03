SELECT description, rating, category
FROM mock_items
WHERE description @@@ 'shoes' OR category @@@ 'footwear' AND rating @@@ '>2'
ORDER BY description
LIMIT 5;
