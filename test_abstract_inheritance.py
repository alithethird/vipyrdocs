#!/usr/bin/env python3
"""
Comprehensive test suite for abstract method inheritance tracking feature.

This script creates various test scenarios to validate:
1. Abstract methods can document Returns/Raises/Yields without errors
2. Cross-file inheritance tracking works correctly
3. Implementations are validated against abstract contracts
4. D070/D071/D072 error codes are properly triggered
"""

import os
import subprocess
import tempfile
import shutil
from pathlib import Path


class TestRunner:
    """Helper class to run vipyrdocs tests."""
    
    def __init__(self, vipyrdocs_path):
        self.vipyrdocs_path = vipyrdocs_path
        self.test_results = []
    
    def run_test(self, test_name, files_dict, expected_errors):
        """
        Run a test case.
        
        Args:
            test_name: Name of the test
            files_dict: Dict of filename -> content
            expected_errors: List of expected error codes (e.g., ['D070', 'D030'])
        """
        print(f"\n{'='*60}")
        print(f"Test: {test_name}")
        print(f"{'='*60}")
        
        # Create temp directory
        with tempfile.TemporaryDirectory() as tmpdir:
            # Write files
            for filename, content in files_dict.items():
                filepath = Path(tmpdir) / filename
                filepath.write_text(content)
                print(f"Created: {filename}")
            
            # Run vipyrdocs
            result = subprocess.run(
                [self.vipyrdocs_path, tmpdir],
                capture_output=True,
                text=True
            )
            
            print(f"\nExit code: {result.returncode}")
            print(f"\nOutput:\n{result.stdout}")
            
            if result.stderr:
                print(f"\nErrors:\n{result.stderr}")
            
            # Check for expected errors
            found_errors = []
            for error_code in expected_errors:
                if error_code in result.stdout:
                    found_errors.append(error_code)
                    print(f"✓ Found expected error: {error_code}")
                else:
                    print(f"✗ Missing expected error: {error_code}")
            
            # Check for unexpected errors
            all_error_codes = ['D010', 'D020', 'D030', 'D031', 'D040', 'D041', 
                             'D050', 'D051', 'D060', 'D070', 'D071', 'D072']
            unexpected = [code for code in all_error_codes 
                         if code in result.stdout and code not in expected_errors]
            
            if unexpected:
                print(f"✗ Unexpected errors: {unexpected}")
            
            success = (set(found_errors) == set(expected_errors) and 
                      len(unexpected) == 0)
            
            self.test_results.append({
                'name': test_name,
                'success': success,
                'found': found_errors,
                'expected': expected_errors,
                'unexpected': unexpected
            })
            
            return success
    
    def print_summary(self):
        """Print test summary."""
        print(f"\n{'='*60}")
        print("TEST SUMMARY")
        print(f"{'='*60}")
        
        passed = sum(1 for t in self.test_results if t['success'])
        total = len(self.test_results)
        
        for result in self.test_results:
            status = "✓ PASS" if result['success'] else "✗ FAIL"
            print(f"{status}: {result['name']}")
            if not result['success']:
                print(f"  Expected: {result['expected']}")
                print(f"  Found: {result['found']}")
                if result['unexpected']:
                    print(f"  Unexpected: {result['unexpected']}")
        
        print(f"\nTotal: {passed}/{total} passed")
        return passed == total


def main():
    # Find vipyrdocs binary
    vipyrdocs_paths = [
        './target/release/vipyrdocs',
        './target/x86_64-unknown-linux-gnu/release/vipyrdocs',
        '../target/release/vipyrdocs',
    ]
    
    vipyrdocs_path = None
    for path in vipyrdocs_paths:
        if os.path.exists(path):
            vipyrdocs_path = path
            break
    
    if not vipyrdocs_path:
        print("Error: Could not find vipyrdocs binary")
        print("Please build it first: cargo build --release")
        return 1
    
    print(f"Using vipyrdocs: {vipyrdocs_path}")
    
    runner = TestRunner(vipyrdocs_path)
    
    # Test 1: Abstract method with Returns - should NOT error
    runner.run_test(
        "Abstract method with Returns section",
        {
            'base.py': '''
from abc import ABC, abstractmethod

class Base(ABC):
    @abstractmethod
    def process(self, data):
        """Process data.
        
        Args:
            data: Input data.
            
        Returns:
            dict: Processed result.
        """
        pass
'''
        },
        []  # No errors expected
    )
    
    # Test 2: Abstract method with Raises - should NOT error
    runner.run_test(
        "Abstract method with Raises section",
        {
            'base.py': '''
from abc import ABC, abstractmethod

class Base(ABC):
    @abstractmethod
    def validate(self, data):
        """Validate data.
        
        Args:
            data: Data to validate.
            
        Raises:
            ValueError: If invalid.
        """
        pass
'''
        },
        []  # No errors expected
    )
    
    # Test 3: Implementation missing Returns - should error with D030 and D070
    runner.run_test(
        "Implementation missing Returns section",
        {
            'base.py': '''
from abc import ABC, abstractmethod

class Base(ABC):
    @abstractmethod
    def process(self, data):
        """Process data.
        
        Returns:
            dict: Result.
        """
        pass
''',
            'impl.py': '''
from base import Base

class Impl(Base):
    def process(self, data):
        """Process implementation.
        
        Args:
            data: Input.
        """
        return {"result": data}
'''
        },
        ['D030', 'D070']  # Both regular and inheritance errors
    )
    
    # Test 4: Implementation missing Raises - should error with D050 and D071
    runner.run_test(
        "Implementation missing Raises section",
        {
            'base.py': '''
from abc import ABC, abstractmethod

class Base(ABC):
    @abstractmethod
    def validate(self, data):
        """Validate.
        
        Raises:
            ValueError: If invalid.
        """
        pass
''',
            'impl.py': '''
from base import Base

class Impl(Base):
    def validate(self, data):
        """Validate implementation."""
        if not data:
            raise ValueError("Invalid")
'''
        },
        ['D050', 'D071']  # Both regular and inheritance errors
    )
    
    # Test 5: Good implementation - should have NO errors
    runner.run_test(
        "Proper implementation with all sections",
        {
            'base.py': '''
from abc import ABC, abstractmethod

class Base(ABC):
    @abstractmethod
    def process(self, data):
        """Process data.
        
        Returns:
            dict: Result.
            
        Raises:
            ValueError: If error.
        """
        pass
''',
            'impl.py': '''
from base import Base

class Impl(Base):
    def process(self, data):
        """Process implementation.
        
        Args:
            data: Input.
            
        Returns:
            dict: Result.
            
        Raises:
            ValueError: If error.
        """
        if not data:
            raise ValueError("Error")
        return {"result": data}
'''
        },
        []  # No errors
    )
    
    # Test 6: Multiple implementations in different files
    runner.run_test(
        "Multiple implementations across files",
        {
            'base.py': '''
from abc import ABC, abstractmethod

class Processor(ABC):
    @abstractmethod
    def process(self, data):
        """Process data.
        
        Returns:
            str: Result.
        """
        pass
''',
            'impl_good.py': '''
from base import Processor

class GoodImpl(Processor):
    def process(self, data):
        """Good implementation.
        
        Returns:
            str: Result.
        """
        return str(data)
''',
            'impl_bad.py': '''
from base import Processor

class BadImpl(Processor):
    def process(self, data):
        """Bad implementation missing Returns."""
        return str(data)
'''
        },
        ['D030', 'D070']  # Only bad implementation should error
    )
    
    # Test 7: Yields section
    runner.run_test(
        "Abstract method with Yields section",
        {
            'base.py': '''
from abc import ABC, abstractmethod

class Base(ABC):
    @abstractmethod
    def generate(self):
        """Generate values.
        
        Yields:
            int: Generated value.
        """
        pass
''',
            'impl.py': '''
from base import Base

class Impl(Base):
    def generate(self):
        """Generate implementation."""
        for i in range(10):
            yield i
'''
        },
        ['D040', 'D072']  # Missing yields documentation
    )
    
    # Test 8: Abstract method with abc.abstractmethod style
    runner.run_test(
        "Abstract method with abc.abstractmethod decorator",
        {
            'base.py': '''
import abc

class Base(abc.ABC):
    @abc.abstractmethod
    def process(self, data):
        """Process data.
        
        Returns:
            str: Result.
        """
        pass
'''
        },
        []  # No errors for abstract method
    )
    
    # Test 9: Multiple base classes
    runner.run_test(
        "Implementation with multiple base classes",
        {
            'base_a.py': '''
from abc import ABC, abstractmethod

class BaseA(ABC):
    @abstractmethod
    def method_a(self):
        """Method A.
        
        Returns:
            int: Value.
        """
        pass
''',
            'base_b.py': '''
from abc import ABC, abstractmethod

class BaseB(ABC):
    @abstractmethod
    def method_b(self):
        """Method B.
        
        Raises:
            RuntimeError: On error.
        """
        pass
''',
            'impl.py': '''
from base_a import BaseA
from base_b import BaseB

class Impl(BaseA, BaseB):
    def method_a(self):
        """Implementation A."""
        return 42
    
    def method_b(self):
        """Implementation B."""
        raise RuntimeError("Error")
'''
        },
        ['D030', 'D070', 'D050', 'D071']  # Missing docs for both methods
    )
    
    # Test 10: Single file with abstract and implementation
    runner.run_test(
        "Abstract and implementation in same file",
        {
            'combined.py': '''
from abc import ABC, abstractmethod

class Base(ABC):
    @abstractmethod
    def process(self):
        """Process.
        
        Returns:
            str: Result.
        """
        pass

class Impl(Base):
    def process(self):
        """Implementation missing Returns."""
        return "done"
'''
        },
        ['D030', 'D070']
    )
    
    # Print summary
    all_passed = runner.print_summary()
    
    return 0 if all_passed else 1


if __name__ == '__main__':
    exit(main())
