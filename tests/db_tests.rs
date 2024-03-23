#[cfg(test)]
mod db_tests {
    use tempfile::tempdir;
    use DPM::db::*;
    use DPM::MyResult;

    #[test]
    fn test_db_new() -> MyResult<()> {
        let dir = tempdir()?;
        let db_path = dir.path();
        let _ = Db::new(db_path)?;
        assert!(db_path.exists());
        Ok(())
    }

    #[test]
    fn test_create_table() -> MyResult<()> {
        let dir = tempdir()?;
        let db_path = dir.path();
        let db = Db::new(db_path)?;
        db.create_table()?;
        Ok(())
    }

    #[test]
    fn test_insert() -> MyResult<()> {
        let dir = tempdir()?;
        let db_path = dir.path();
        let db = Db::new(db_path)?;
        db.create_table()?;
        let pkg = DbPackage::new(
            "test_pkg",
            "0.1.0",
            "http://example.com",
            "A test package",
            "test_pkg.tar.gz",
            "1234567890abcdef",
            "bin/test_pkg",
        );
        db.insert(pkg)?;
        Ok(())
    }

    #[test]
    fn test_read_all() -> MyResult<()> {
        let dir = tempdir()?;
        let db_path = dir.path();
        let db = Db::new(db_path)?;
        db.create_table()?;
        let pkg = DbPackage::new(
            "test_pkg",
            "0.1.0",
            "http://example.com",
            "A test package",
            "test_pkg.tar.gz",
            "1234567890abcdef",
            "bin/test_pkg",
        );
        db.insert(pkg.clone())?;
        let all = db.read_all()?;
        assert!(all.contains_key("test_pkg"));
        assert_eq!(all.get("test_pkg").unwrap(), &pkg);
        assert_eq!(all.len(), 1);

        Ok(())
    }

    #[test]
    fn test_read_one() -> MyResult<()> {
        let dir = tempdir()?;
        let db_path = dir.path();
        let db = Db::new(db_path)?;
        db.create_table()?;
        let pkg = DbPackage::new(
            "test_pkg",
            "0.1.0",
            "http://example.com",
            "A test package",
            "test_pkg.tar.gz",
            "1234567890abcdef",
            "bin/test_pkg",
        );
        db.insert(pkg.clone())?;
        let all = db.read_one("test_pkg")?;
        assert!(all.contains_key("test_pkg"));
        assert_eq!(all.get("test_pkg").unwrap(), &pkg);
        assert_eq!(all.len(), 1);

        Ok(())
    }

    #[test]
    fn test_read_one_field() -> MyResult<()> {
        let dir = tempdir()?;
        let db_path = dir.path();
        let db = Db::new(db_path)?;
        db.create_table()?;
        let pkg = DbPackage::new(
            "test_pkg",
            "0.1.0",
            "http://example.com",
            "A test package",
            "test_pkg.tar.gz",
            "1234567890abcdef",
            "bin/test_pkg",
        );
        db.insert(pkg.clone())?;
        let all = db.read_one_field("name", "test_pkg")?;
        assert_eq!(all, "test_pkg");

        Ok(())
    }
}
