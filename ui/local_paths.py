"""Keep calculator data isolated from the dashboard and trading installation."""
import os
from pathlib import Path


def data_dir():
    base = Path(os.environ['LOCALAPPDATA']) if os.name == 'nt' else Path(
        os.environ.get('XDG_DATA_HOME') or Path.home() / '.local/share')
    return base / 'IndependentInvestmentCalculator'


def korean_font(bold=False):
    if os.name == 'nt':
        return 'C:/Windows/Fonts/' + ('malgunbd.ttf' if bold else 'malgun.ttf')
    base = Path(os.environ.get('XDG_DATA_HOME') or Path.home() / '.local/share')
    filename = 'NotoSansCJKkr-Bold.otf' if bold else 'NotoSansCJKkr-Regular.otf'
    return str(base / 'fonts/steamdeck-5814' / filename)
