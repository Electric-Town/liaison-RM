import type { B0Route } from "../application-contract";

export interface RouteNavigationItem {
  readonly id: B0Route;
  readonly label: string;
}

export interface RouteNavigationProps {
  readonly current: B0Route;
  readonly items: readonly RouteNavigationItem[];
  readonly onNavigate: (route: B0Route) => void;
}

export function RouteNavigation({ current, items, onNavigate }: RouteNavigationProps) {
  return (
    <nav aria-label="Primary" className="lrm-route-navigation">
      {items.map((item) => (
        <button
          aria-current={item.id === current ? "page" : undefined}
          className="lrm-route-navigation__item"
          key={item.id}
          onClick={() => onNavigate(item.id)}
          type="button"
        >
          {item.label}
        </button>
      ))}
    </nav>
  );
}
