# Testing Abstract Method Documentation Feature

## Quick Start

This guide will help you test the new abstract method documentation tracking feature in vipyrdocs.

## Installation

### Option 1: Use Pre-built Binary (Linux x64)

Download the `vipyrdocs` executable from the release and make it executable:

```bash
chmod +x vipyrdocs
./vipyrdocs --version
```

### Option 2: Build from Source

```bash
git clone https://github.com/alithethird/vipyrdocs.git
cd vipyrdocs
git checkout copilot/track-abstract-function-docs
cargo build --release
```

The binary will be at `target/release/vipyrdocs`

## What's New?

This feature adds two major improvements:

1. **Abstract methods can now document Returns/Raises/Yields** without getting false positive errors
2. **Cross-file inheritance tracking** ensures implementations match their abstract contracts

## Testing the Feature

### Step 1: Create Test Files

Create a test directory with Python files:

```bash
mkdir -p ~/vipyrdocs_test
cd ~/vipyrdocs_test
```

Create `base.py`:
```python
"""Abstract base class."""
from abc import ABC, abstractmethod

class DataProcessor(ABC):
    """Abstract base for data processors."""
    
    @abstractmethod
    def validate(self, data):
        """Validate input data.
        
        Args:
            data: Data to validate.
            
        Returns:
            bool: True if valid.
            
        Raises:
            ValueError: If data is invalid.
        """
        pass
    
    @abstractmethod
    def transform(self, data):
        """Transform data.
        
        Args:
            data: Input data.
            
        Returns:
            dict: Transformed result.
        """
        pass
```

Create `good_impl.py`:
```python
"""Good implementation - properly documented."""
from base import DataProcessor

class CSVProcessor(DataProcessor):
    """CSV processor with complete docs."""
    
    def validate(self, data):
        """Validate CSV data.
        
        Args:
            data: CSV data.
            
        Returns:
            bool: True if valid.
            
        Raises:
            ValueError: If data is invalid.
        """
        if not data:
            raise ValueError("Empty data")
        return True
    
    def transform(self, data):
        """Transform CSV to dict.
        
        Args:
            data: CSV data.
            
        Returns:
            dict: Transformed result.
        """
        return {"data": data}
```

Create `bad_impl.py`:
```python
"""Bad implementation - missing docs."""
from base import DataProcessor

class JSONProcessor(DataProcessor):
    """JSON processor missing docs."""
    
    def validate(self, data):
        """Validate JSON data.
        
        Args:
            data: JSON data.
        """
        # Missing Returns and Raises sections!
        if not data:
            raise ValueError("Empty data")
        return True
    
    def transform(self, data):
        """Transform JSON to dict.
        
        Args:
            data: JSON data.
        """
        # Missing Returns section!
        return {"json": data}
```

### Step 2: Run vipyrdocs

Run the tool on the test directory:

```bash
./vipyrdocs ~/vipyrdocs_test
```

### Step 3: Expected Output

You should see output like this:

```
🐍 Scanning path: /home/user/vipyrdocs_test
🐍 Scan result:
  🚨 /home/user/vipyrdocs_test/bad_impl.py:
  - 16:8 D030 function/ method that returns a value should have the returns section in the docstring
  - 25:8 D030 function/ method that returns a value should have the returns section in the docstring
  - 15:12 D050 a function/ method that raises an exception should have the raises section in the docstring
  - 7:0 D070 method 'validate' in class 'JSONProcessor' implements abstract method from 'DataProcessor' which documents a return value, but this implementation is missing a Returns section in the docstring
  - 7:0 D071 method 'validate' in class 'JSONProcessor' implements abstract method from 'DataProcessor' which documents exceptions, but this implementation is missing a Raises section in the docstring
  - 18:0 D070 method 'transform' in class 'JSONProcessor' implements abstract method from 'DataProcessor' which documents a return value, but this implementation is missing a Returns section in the docstring
📊 Summary: scanned 3 files; 1 had issues; 6 issues total.
```

### What to Look For

✅ **base.py has NO errors** - Abstract methods can document Returns/Raises without implementation

✅ **good_impl.py has NO errors** - Properly documented implementation

❌ **bad_impl.py has 6 errors**:
- 3 regular errors (D030, D050) - methods that return/raise without docs
- 3 **new inheritance errors (D070, D071)** - missing docs required by abstract base

## Understanding the Errors

### New Error Codes

- **D070**: Implementation missing Returns section required by abstract method
- **D071**: Implementation missing Raises section required by abstract method  
- **D072**: Implementation missing Yields section required by abstract method

### Key Points

1. **Abstract methods don't trigger D031/D051/D041** anymore (false positives fixed)
2. **Cross-file tracking** - works even when base and implementation are in different files
3. **Both checks run** - you get regular errors (D030/D050) AND inheritance errors (D070/D071)

## Single File Test

You can also test with a single file:

Create `single_file_test.py`:
```python
from abc import abstractmethod

class Base:
    @abstractmethod
    def process(self):
        """Process data.
        
        Returns:
            str: Result.
        """
        pass

class Impl(Base):
    def process(self):
        """Process implementation."""
        return "done"  # Missing Returns section!
```

Run:
```bash
./vipyrdocs single_file_test.py
```

Expected:
```
  - 17:8 D030 function/ method that returns a value should have the returns section in the docstring
  - 14:0 D070 method 'process' in class 'Impl' implements abstract method from 'Base' which documents a return value, but this implementation is missing a Returns section in the docstring
```

## Questions?

If you encounter any issues:
1. Make sure you're using Python 3 syntax in your test files
2. Check that abstract methods use `@abstractmethod` decorator
3. Verify the tool scans all `.py` files in the directory

Happy testing! 🐍
