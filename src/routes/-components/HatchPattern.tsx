interface HatchPatternProps extends React.ComponentProps<"pattern"> {
  id: string;
  color: string;
  spacing?: number;
}

const HatchPattern = ({ id, color, spacing = 5, ...props }: HatchPatternProps) => {
  return (
    <pattern
      id={id}
      width={spacing}
      height={spacing}
      patternUnits="userSpaceOnUse"
      patternTransform="rotate(45)"
      {...props}
    >
      <rect width={spacing} height={spacing} style={{ fill: color, fillOpacity: 0.2 }} />
      <line x1={0} y1={0} x2={0} y2={spacing} style={{ stroke: color, strokeWidth: 2.5, strokeOpacity: 0.75 }} />
    </pattern>
  );
};

export default HatchPattern;
