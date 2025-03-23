import React, { useRef, useEffect } from 'react';
import GridStack, { GridStackNode, GridStackOptions } from 'gridstack';
import 'gridstack/dist/gridstack.min.css';

interface GridItemProps {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  content: React.ReactNode;
}

interface GridStackComponentProps {
  items: GridItemProps[];
  options?: GridStackOptions;
  onItemChange?: (items: GridStackNode[]) => void;
}

const GridStackComponent: React.FC<GridStackComponentProps> = ({ items, options, onItemChange }) => {
  const gridRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!gridRef.current) return;

    const grid = GridStack.(
      {
        ...options,
        float: true, // Enable dragging and resizing
        resizable: { handles: 'se, sw, ne, nw, n, e, s, w' },
        draggable: { handle: '.grid-stack-item-content' },
      },
      gridRef.current
    );

    items.forEach((item) => {
      grid.addWidget({
        x: item.x,
        y: item.y,
        w: item.width,
        h: item.height,
        id: item.id,
        content: `<div class="grid-stack-item-content">${React.isValidElement(item.content) ? '<div id="react-content-'+item.id+'"></div>' : item.content}</div>`,
      });

      if(React.isValidElement(item.content)){
        const reactContainer = document.getElementById("react-content-"+item.id);
        if(reactContainer){
          React.render(item.content, reactContainer);
        }
      }
    });

    const handleChange = (event, changedItems) => {
      if (onItemChange) {
        onItemChange(changedItems);
      }
    };

    grid.on('change', handleChange);

    return () => {
      grid.off('change', handleChange);
      grid.destroy();
    };
  }, [items, options, onItemChange]);

  return <div ref={gridRef} className="grid-stack" />;
};

const ExampleGrid = () => {
  const [gridItems, setGridItems] = React.useState<GridItemProps[]>([
    { id: '1', x: 0, y: 0, width: 2, height: 2, content: <div>Item 1</div> },
    { id: '2', x: 2, y: 1, width: 1, height: 1, content: <div>Item 2</div> },
    { id: '3', x: 3, y: 3, width: 3, height: 2, content: <button>Click Me</button>},
  ]);

  const handleItemChange = (changedItems: GridStackNode[]) => {
    const updatedItems = changedItems.map((item) => ({
      id: item.id as string,
      x: item.x || 0,
      y: item.y || 0,
      width: item.w || 1,
      height: item.h || 1,
      content: gridItems.find((i) => i.id === item.id)?.content || <div></div>,
    }));
    setGridItems(updatedItems);
    console.log('Grid items changed:', updatedItems);
  };

  return (
    <GridStackComponent
      items={gridItems}
      onItemChange={handleItemChange}
      options={{ column: 12, resizable: { handles: 'se, sw, ne, nw, n, e, s, w' } }}
    />
  );
};

export default ExampleGrid;