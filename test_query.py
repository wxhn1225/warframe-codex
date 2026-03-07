import sqlite3, sys
sys.stdout.reconfigure(encoding='utf-8')

conn = sqlite3.connect('warframe.db')
conn.row_factory = sqlite3.Row

print("=== 测试：利润收割者圆蛛 ===")
rows = conn.execute(
    "SELECT e.path, e.export_file, e.category, l.text as name_zh "
    "FROM export_entries e "
    "JOIN localizations l ON l.lang_key = e.name_key AND l.lang_code = 'zh' "
    "WHERE l.text LIKE '%圆蛛%'"
).fetchall()

for r in rows:
    print(f"  路径: {r['path']}")
    print(f"  来源: {r['export_file']} / {r['category']}")
    print(f"  中文名: {r['name_zh']}")
    print()

print("=== 利润收割者圆蛛的所有关系 ===")
if rows:
    path = rows[0]['path']
    rels = conn.execute(
        "SELECT rel_type, to_path FROM relations WHERE from_path = ? LIMIT 15",
        (path,)
    ).fetchall()
    for rel in rels:
        name = conn.execute(
            "SELECT l.text FROM export_entries e "
            "JOIN localizations l ON l.lang_key = e.name_key AND l.lang_code = 'zh' "
            "WHERE e.path = ?",
            (rel['to_path'],)
        ).fetchone()
        label = name[0] if name else rel['to_path'][:70]
        print(f"  [{rel['rel_type']}] → {label}")

print()
print("=== 多语言对照 ===")
multi = conn.execute(
    "SELECT l_zh.text zh, l_en.text en, l_ja.text ja, l_ko.text ko "
    "FROM export_entries e "
    "JOIN localizations l_zh ON l_zh.lang_key = e.name_key AND l_zh.lang_code = 'zh' "
    "JOIN localizations l_en ON l_en.lang_key = e.name_key AND l_en.lang_code = 'en' "
    "JOIN localizations l_ja ON l_ja.lang_key = e.name_key AND l_ja.lang_code = 'ja' "
    "JOIN localizations l_ko ON l_ko.lang_key = e.name_key AND l_ko.lang_code = 'ko' "
    "WHERE l_zh.text LIKE '%圆蛛%'"
).fetchall()
for r in multi:
    print(f"  简中: {r['zh']}")
    print(f"  英文: {r['en']}")
    print(f"  日文: {r['ja']}")
    print(f"  韩文: {r['ko']}")
    print()

print("=== 实体类型分布（前10）===")
for row in conn.execute(
    "SELECT file_type, COUNT(*) n FROM entities GROUP BY file_type ORDER BY n DESC LIMIT 10"
):
    print(f"  {row[0]}: {row[1]:,}")

print()
print("=== 关系类型分布（前15）===")
for row in conn.execute(
    "SELECT rel_type, COUNT(*) n FROM relations GROUP BY rel_type ORDER BY n DESC LIMIT 15"
):
    print(f"  {row[0]}: {row[1]:,}")
