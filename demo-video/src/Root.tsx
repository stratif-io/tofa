import { Composition } from "remotion";
import { ScanCamTour, TOTAL_FRAMES } from "./compositions/Tour";
// Side-effect import: registers Google Fonts with Remotion before any frame
// is rendered, so JetBrains Mono / Inter / Fraunces are guaranteed available.
import "./theme/tokens";

export const RemotionRoot: React.FC = () => {
  return (
    <Composition
      id="ScanCamTour"
      component={ScanCamTour}
      durationInFrames={TOTAL_FRAMES}
      fps={30}
      width={1280}
      height={800}
    />
  );
};
