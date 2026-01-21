#!/bin/bash

# This script fixes all MoveRecord creations in history.rs by adding piece field

cat > /tmp/fix_history.py << 'EOF'
import re
import sys

content = sys.stdin.read()

# Pattern to find MoveRecord { ... } (across multiple lines)
pattern = r'(MoveRecord\s*\{[^}]*\})'

def add_piece_field(match):
    move_record = match.group(1)
    # Check if piece field already exists
    if 'piece:' in move_record:
        return move_record
    
    # Add piece field (default General, Red for simplicity)
    move_record = move_record.replace('MoveRecord {', 'MoveRecord {\n            piece: Piece::new(PieceType::General, Color::Red),')
    return move_record

# Apply the fix
fixed_content = re.sub(pattern, add_piece_field, content, flags=re.DOTALL)

print(fixed_content)
EOF

python3 /tmp/fix_history.py < src/history.rs > src/history_fixed.rs
mv src/history_fixed.rs src/history.rs

# Also need to update the import to include Piece and PieceType
sed -i '1s/^/use crate::piece::{Piece, PieceType, Color};\n/' src/history.rs

echo "Fixed MoveRecord struct in history.rs"